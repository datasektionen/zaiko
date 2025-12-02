use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    get,
    http::header::Header,
    web, HttpMessage, HttpResponse,
};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use jsonwebtoken::get_current_timestamp;
use openidconnect::{
    core::{
        CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm,
    },
    AccessTokenHash, AuthorizationCode, CsrfToken, EmptyAdditionalClaims, IdToken, IdTokenClaims,
    IdTokenVerifier, OAuth2TokenResponse, TokenResponse,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use types::{
    AuthMiddleware, AuthTokenResponse, InnerAuthMiddleware, LocalBoxFuture, OIDCData, Token,
};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    auth::types::{Group, HivePermission, UserInfo},
    db,
    error::Error,
};

pub mod types;

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
    state: String,
}

pub(crate) fn config() -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut ServiceConfig| {
        cfg.service(scope("/oidc").service(callback));
    }
}

#[derive(Debug)]
pub enum CheckType<'a> {
    Any,
    Storage {
        storage: &'a str,
        container: Option<&'a str>,
    },
    Item(&'a str),
    MoveItem {
        from_storage: &'a str,
        from_container: &'a str,
        to_storage: &'a str,
        to_container: &'a str,
    },
    MoveContainer {
        container: &'a str,
        from_storage: &'a str,
        to_storage: &'a str,
    },
    Admin,
    SupplierCreate {
        mandates: &'a [Group],
        mandate: &'a str,
    },
    Supplier {
        mandates: &'a [Group],
        name: &'a str,
    },
}

/// Checks if user is allowed to perform an action based on provided information
///
/// storage or container:
///     Allowed if the user has write access to that location
/// item:
///     Allowed if the user has write access to any location that item is stored
/// none:
///     Allowed if the user has write access to any location
pub(crate) async fn check_auth(
    check_type: CheckType<'_>,
    db: &Pool<Postgres>,
    permissions: &[HivePermission],
) -> Result<(), Error> {
    // TODO: check protected
    log::debug!("Checking permission {:?}", check_type);

    if permissions.iter().any(|perm| perm.id == "admin") {
        return Ok(());
    }

    if match check_type {
        CheckType::Any => permissions.iter().any(|perm| perm.id == "write"),
        CheckType::Storage {
            storage,
            container: Some(container),
        } => permissions.iter().any(|perm| {
            perm.id == "write"
                && (perm.scope == Some(storage.to_lowercase())
                    || perm.scope == Some(container.to_lowercase()))
        }),
        CheckType::Storage {
            storage,
            container: None,
        } => permissions
            .iter()
            .any(|perm| perm.id == "write" && perm.scope == Some(storage.to_lowercase())),
        CheckType::Item(item) => db::item::get_location(db, &item).await?.iter().any(|item| {
            permissions.iter().any(|perm| {
                perm.id == "write"
                    && (perm.scope == Some(item.storage.to_lowercase())
                        || perm.scope == Some(item.container.to_lowercase()))
            })
        }),
        CheckType::MoveItem {
            from_storage,
            from_container,
            to_storage,
            to_container,
        } => {
            permissions.iter().any(|perm| {
                perm.id == "write"
                    && (perm.scope == Some(from_storage.to_lowercase())
                        || perm.scope == Some(from_container.to_lowercase()))
            }) && permissions.iter().any(|perm| {
                perm.id == "write"
                    && (perm.scope == Some(to_storage.to_lowercase())
                        || perm.scope == Some(to_container.to_lowercase()))
            })
        }
        CheckType::MoveContainer {
            container,
            from_storage,
            to_storage,
        } => {
            permissions
                .iter()
                .any(|perm| perm.id == "write" && perm.scope == Some(container.to_lowercase()))
                || permissions.iter().any(|perm| {
                    perm.id == "write" && (perm.scope == Some(from_storage.to_lowercase()))
                }) && permissions.iter().any(|perm| {
                    perm.id == "write" && (perm.scope == Some(to_storage.to_lowercase()))
                })
        }
        CheckType::SupplierCreate { mandates, mandate } => {
            permissions.iter().any(|perm| perm.id == "write")
                && mandates.iter().any(|man| man.0 == mandate)
        }
        CheckType::Supplier { mandates, name } => {
            let supplier = db::supplier::get_by_name(db, name).await?;
            mandates.iter().any(|mandate| mandate.0 == supplier.mandate)
        }
        CheckType::Admin => permissions.iter().any(|perm| perm.id == "admin"),
    } {
        Ok(())
    } else {
        Err(Error::Unauthorized)
    }
}

pub async fn get_permitted_storages(
    db: &Pool<Postgres>,
    permissions: &[HivePermission],
) -> Result<Vec<String>, Error> {
    if check_auth(CheckType::Admin, db, permissions).await.is_ok() {
        Ok(db::storage::get_all_unprotected(db).await.map(|storages| {
            storages
                .iter()
                .map(|storage| storage.name.clone().to_lowercase())
                .collect::<Vec<String>>()
        })?)
    } else {
        Ok(permissions
            .iter()
            .filter_map(|perm| {
                if perm.id == "write" {
                    perm.scope.clone()
                } else {
                    None
                }
            })
            .collect())
    }
}

#[utoipa::path(
    tag = "auth",
    responses(
        (status = 200, body = UserInfo, description = "Info about the currently logged in user")
    )
)]
#[get("/userinfo")]
async fn user_info(
    id: web::ReqData<String>,
    permissions: web::ReqData<Vec<HivePermission>>,
    groups: web::ReqData<Vec<Group>>,
) -> HttpResponse {
    let info = UserInfo {
        username: id.to_string(),
        permissions: permissions.to_vec(),
        groups: groups.to_vec(),
        // TODO: use rfinger to get accutal picture
        image: String::new(),
    };
    HttpResponse::Ok().json(info)
}

#[utoipa::path(
    tag = "auth",
    responses(
        (status = 200, description = "OIDC callback function")
    )
)]
#[get("/callback")]
async fn callback(
    query: web::Query<CallbackQuery>,
    oidc: web::Data<OIDCData>,
) -> Result<HttpResponse, Error> {
    let CallbackQuery { code, state } = query.0;
    let OIDCData {
        client,
        http_client,
        nonce,
        csrf_token,
    } = oidc.get_ref();

    check_csrf_token(csrf_token, state)?;

    let token_respones = client
        .exchange_code(AuthorizationCode::new(code))?
        .request_async(http_client)
        .await?;

    let id_token = token_respones
        .id_token()
        .ok_or(Error::InternalServerError(String::from(
            "oidc server returned no id",
        )))?;

    let id_token_verifier = client.id_token_verifier();

    let claims = id_token.claims(&id_token_verifier, nonce)?;

    check_token_hash(claims, &token_respones, id_token, id_token_verifier)?;

    let permissions = HivePermission::get(claims.subject().to_string()).await?;

    log::debug!("permissions: {permissions:?}");

    let groups = Group::get(
        &claims.subject().to_string(),
        permissions.contains(&HivePermission {
            id: String::from("admin"),
            scope: Some(String::from("")),
        }),
    )
    .await?;

    log::debug!("groups: {:?}", groups);

    let token = Token::new(claims.subject().to_string(), permissions, groups).unwrap();

    let cookie = token.cookie()?;

    Ok(HttpResponse::TemporaryRedirect()
        .insert_header(("location", "/"))
        .cookie(cookie)
        .finish())
}

fn check_csrf_token(token: &CsrfToken, state: String) -> Result<(), Error> {
    if *token.secret() != state {
        log::error!("Invalid CSRF token on oidc callback");
        return Err(Error::BadRequest);
    }

    Ok(())
}

fn check_token_hash(
    claims: &IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>,
    token_respones: &AuthTokenResponse,
    id_token: &IdToken<
        EmptyAdditionalClaims,
        CoreGenderClaim,
        CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm,
    >,
    id_token_verifier: IdTokenVerifier<'_, CoreJsonWebKey>,
) -> Result<(), Error> {
    let expected_access_token_hash =
        claims
            .access_token_hash()
            .ok_or(Error::InternalServerError(String::from(
                "Missing access token hash",
            )))?;

    let actual_access_token_hash = AccessTokenHash::from_token(
        token_respones.access_token(),
        id_token.signing_alg()?,
        id_token.signing_key(&id_token_verifier)?,
    )?;

    if actual_access_token_hash != *expected_access_token_hash {
        return Err(Error::InternalServerError(format!(
            "Hashes did not mach for subject {}",
            **claims.subject()
        )));
    }

    Ok(())
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Transform = InnerAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InnerAuthMiddleware {
            service: Rc::new(service),
            auth_url: self.auth_url.clone(),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for InnerAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        let mut token = if let Some(token) = Token::extract_token(req.cookie("token")) {
            token
        } else {
            if let Ok(auth) = Authorization::<Bearer>::parse(&req) {
                return Box::pin(async move {
                    let permissions =
                        HivePermission::get_from_api_token(auth.into_scheme().token().to_string())
                            .await;

                    if let Err(error) = &permissions {
                        log::error!("{error}");
                    }

                    let permissions = permissions.unwrap_or(Vec::new());

                    let token = Token::new(String::from("token"), permissions, Vec::new()).unwrap();

                    req.extensions_mut().insert(String::from("api key"));
                    req.extensions_mut().insert(token.permissions.clone());
                    req.extensions_mut().insert(token.groups.clone());

                    let response = srv.call(req);

                    let res = response.await.unwrap();

                    Ok(res.map_into_left_body())
                });
            }

            let response = HttpResponse::TemporaryRedirect()
                .insert_header(("location", self.auth_url.clone()))
                .finish()
                .map_into_right_body();

            let (request, _pl) = req.into_parts();
            return Box::pin(async move { Ok(ServiceResponse::new(request, response)) });
        };

        req.extensions_mut().insert(token.sub.clone());
        req.extensions_mut().insert(token.permissions.clone());
        req.extensions_mut().insert(token.groups.clone());

        let res = srv.call(req);

        Box::pin(async move {
            let mut res = res.await.unwrap();

            token.exp = get_current_timestamp() + 7200;
            let cookie = token.cookie().unwrap();
            res.response_mut().add_cookie(&cookie).unwrap();

            Ok(res.map_into_left_body())
        })
    }
}
