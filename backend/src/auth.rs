use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    get, post, web, HttpMessage, HttpResponse,
};
use jsonwebtoken::get_current_timestamp;
use openidconnect::{
    core::{
        CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm,
    },
    AccessTokenHash, AuthorizationCode, CsrfToken, EmptyAdditionalClaims, IdToken, IdTokenClaims,
    IdTokenVerifier, OAuth2TokenResponse, TokenResponse,
};
use serde::Deserialize;
use std::{
    env,
    future::{ready, Ready},
};
use types::{
    AuthMiddleware, AuthTokenResponse, Club, ClubGetResponse, InnerAuthMiddleware, LocalBoxFuture,
    OIDCData, Permission, Token,
};

use crate::error::Error;

pub mod types;

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
    state: String,
}

#[get("/api/oidc/callback")]
pub async fn auth_callback(
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

    let permissions = Permission::get_pls_permissions(claims.subject().to_string()).await?;

    let token = Token::new(claims.subject().to_string(), permissions).unwrap();

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

#[get("/clubs")]
pub async fn get_clubs(token: Token) -> Result<HttpResponse, Error> {
    let clubs: Vec<Club> = token.permissions.into_iter().map(Club::from).collect();

    let res = ClubGetResponse {
        active: Club::from((token.active_club, token.active_permission)),
        clubs,
    };

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
struct ClubQuery {
    club: String,
}

#[post("/club")]
pub async fn set_club(
    mut token: Token,
    query: web::Query<ClubQuery>,
) -> Result<HttpResponse, Error> {
    let club = query.club.clone();
    token.set_active_club(club)?;
    let cookie = token.cookie()?;
    Ok(HttpResponse::Ok().cookie(cookie).finish())
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
            service,
            permission: self.permission.clone(),
            auth_url: self.auth_url.clone(),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for InnerAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_owned();
        if env::var("APP_AUTH") == Ok(String::from("false"))
            && Token::extract_token(req.cookie("token")).is_none()
        {
            return fake_auth(req, self);
        }

        if req.path().contains("oidc") || req.path().contains("static") {
            let res = self.service.call(req);
            return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
        }

        if let Some(mut token) = Token::extract_token(req.cookie("token")) {
            if check_permission(self.permission.clone(), token.clone()) {
                return auth_error_response(req, self.auth_url.clone());
            }

            req.extensions_mut().insert(token.active_club.clone());

            let res = self.service.call(req);

            Box::pin(async move {
                let mut res = res.await.unwrap();

                if !path.contains("club") {
                    token.exp = get_current_timestamp() + 7200;
                    let cookie = token.cookie().unwrap();
                    res.response_mut().add_cookie(&cookie).unwrap();
                }

                Ok(res.map_into_left_body())
            })
        } else {
            auth_error_response(req, self.auth_url.clone())
        }
    }
}

fn check_permission(permission: Permission, token: Token) -> bool {
    match permission {
        Permission::Read => !matches!(
            token.active_permission,
            Permission::Read | Permission::ReadWrite
        ),
        Permission::ReadWrite => !matches!(token.active_permission, Permission::ReadWrite),
    }
}

fn fake_auth<B, S>(
    req: ServiceRequest,
    auth: &InnerAuthMiddleware<S>,
) -> LocalBoxFuture<Result<ServiceResponse<EitherBody<B>>, actix_web::Error>>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    log::debug!("fake auth");
    let permissions = Permission::default_privlages();
    let token = Token::new(String::from("ture"), permissions).unwrap();

    req.extensions_mut().insert(token.active_club.clone());

    let res = auth.service.call(req);

    Box::pin(async move {
        let cookie = token.cookie().unwrap();
        let mut res = res.await.unwrap();
        res.response_mut().add_cookie(&cookie).unwrap();

        Ok(res.map_into_left_body())
    })
}

fn auth_error_response<B>(
    req: ServiceRequest,
    auth_url: String,
) -> LocalBoxFuture<Result<ServiceResponse<EitherBody<B>>, actix_web::Error>>
where
    B: 'static,
{
    if env::var("APP_AUTH") == Ok(String::from("false")) {
        let response = HttpResponse::Unauthorized().finish().map_into_right_body();
        let (request, _pl) = req.into_parts();
        Box::pin(async move { Ok(ServiceResponse::new(request, response)) })
    } else {
        let response = HttpResponse::TemporaryRedirect()
            .insert_header(("location", auth_url))
            .finish()
            .map_into_right_body();

        let (request, _pl) = req.into_parts();
        Box::pin(async move { Ok(ServiceResponse::new(request, response)) })
    }
}
