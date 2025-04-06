use actix_web::{
    body::{EitherBody, MessageBody},
    cookie::{Cookie, SameSite},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    get, post, web, FromRequest, HttpMessage, HttpRequest, HttpResponse,
};
use derive_more::Display;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use openidconnect::{
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreAuthenticationFlow, CoreErrorResponseType,
        CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreRevocableToken, CoreTokenType,
    },
    reqwest, AccessTokenHash, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken,
    EmptyAdditionalClaims, EmptyExtraTokenFields, EndpointMaybeSet, EndpointNotSet, EndpointSet,
    IdToken, IdTokenClaims, IdTokenFields, IdTokenVerifier, IssuerUrl, Nonce, OAuth2TokenResponse,
    RedirectUrl, RevocationErrorResponseType, StandardErrorResponse,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenResponse,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    future::{ready, Future, Ready},
    pin::Pin,
};

use crate::error::Error;

type OIDCClient = openidconnect::Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    StandardTokenResponse<
        IdTokenFields<
            EmptyAdditionalClaims,
            EmptyExtraTokenFields,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
        >,
        CoreTokenType,
    >,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, CoreTokenType>,
    CoreRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

#[derive(Clone)]
pub struct OIDCData {
    client: OIDCClient,
    http_client: reqwest::Client,
    // pkce_verifier: PkceCodeVerifier,
    nonce: Nonce,
    csrf_token: CsrfToken,
}

#[derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "r")]
    Read,
    #[serde(rename = "rw")]
    ReadWrite,
}

pub async fn get_oidc() -> (OIDCData, String) {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(env::var("OIDC_PROVIDER").expect("OIDC_PROVIDER in .env"))
            .expect("OIDC_PROVIDER to be correctly formated"),
        &http_client,
    )
    .await
    .expect("to get metadata from oidc provider");

    let client = Client::from_provider_metadata(
        provider_metadata,
        ClientId::new(env::var("OIDC_ID").expect("OIDC_ID in .env")),
        Some(ClientSecret::new(
            env::var("OIDC_SECRET").expect("OIDC_SECRET in .env"),
        )),
    )
    .set_redirect_uri(
        RedirectUrl::new(env::var("REDIRECT_URL").expect("REDIRECT_URL in .env"))
            .expect("REDIRECT_URL to be correctly formated"),
    );

    // let (pkce_challange, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // .add_scope(Scope::new(String::from("pls_zaiko")))
        // .set_pkce_challenge(pkce_challange)
        .url();

    let oidc = OIDCData {
        client,
        http_client,
        nonce,
        // pkce_verifier,
        csrf_token,
    };

    (oidc, auth_url.to_string())
}

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

    let permissions = get_pls_permissions(claims.subject().to_string()).await?;

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
    token_respones: &StandardTokenResponse<
        IdTokenFields<
            EmptyAdditionalClaims,
            EmptyExtraTokenFields,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
        >,
        CoreTokenType,
    >,
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

async fn get_pls_permissions(subject: String) -> Result<HashMap<String, Permission>, Error> {
    let pls_url = env::var("PLS_URL")?;

    let res = reqwest::get(format!("{}/user/{}/zaiko", pls_url, subject.as_str()))
        .await?
        .text()
        .await?;

    let mut privlages: HashMap<String, Permission> = default_privlages();

    let pls_permissions = serde_json::from_str::<Vec<String>>(&res)?;

    for privlage in pls_permissions {
        let mut permission = privlage.split("-");
        let name = permission
            .next()
            .ok_or(Error::InternalServerError(String::from(
                "Incorrect formating och pls permission",
            )))?
            .to_string();
        let permission = match permission
            .next()
            .ok_or(Error::InternalServerError(String::from(
                "Incorrect formatting of pls permission",
            )))? {
            "r" => Permission::Read,
            "rw" => Permission::ReadWrite,
            _ => {
                return Err(Error::InternalServerError(String::from(
                    "pls permission contains incorrect access type",
                )))
            }
        };
        log::debug!("{}:{}", name, permission);
        privlages.insert(name, permission);
    }

    Ok(privlages)
}

fn default_privlages() -> HashMap<String, Permission> {
    let mut privlages = HashMap::new();

    privlages.insert(String::from("metadorerna"), Permission::Read);

    privlages
}

#[get("/clubs")]
pub async fn get_clubs(token: Token) -> Result<HttpResponse, Error> {
    let clubs: Vec<Club> = token.permissions.into_iter().map(Club::from).collect();

    Ok(HttpResponse::Ok().json(clubs))
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Club {
    name: String,
    permission: Permission,
}

impl From<(String, Permission)> for Club {
    fn from(value: (String, Permission)) -> Self {
        Club {
            name: value.0,
            permission: value.1,
        }
    }
}

impl From<(&String, &Permission)> for Club {
    fn from(value: (&String, &Permission)) -> Self {
        Club {
            name: value.0.to_string(),
            permission: value.1.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub sub: String,
    pub exp: u64,
    pub permissions: HashMap<String, Permission>,
    pub active_club: String,
    pub active_permission: Permission,
}

impl Token {
    fn new(sub: String, permissions: HashMap<String, Permission>) -> Option<Self> {
        let active: Club = permissions.iter().next()?.into();

        Some(Token {
            sub,
            exp: get_current_timestamp() + 7200,
            permissions,
            active_club: active.name,
            active_permission: active.permission,
        })
    }

    fn cookie(&self) -> Result<Cookie, Error> {
        let secret = EncodingKey::from_secret(env::var("APP_SECRET")?.as_bytes());

        let token = encode(&Header::new(Algorithm::HS256), &self, &secret).unwrap();

        Ok(Cookie::build("token", token)
            .same_site(SameSite::Lax)
            .secure(true)
            .http_only(true)
            .path("/")
            .finish())
    }

    fn set_active_club(&mut self, club: String) -> Result<(), Error> {
        let permission = self.permissions.get(&club).ok_or(Error::Unauthorized)?;
        self.active_club = club;
        self.active_permission = permission.clone();
        Ok(())
    }
}

impl FromRequest for Token {
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let cookie = req.cookie("token");
        Box::pin(async move {
            let token = extract_token(cookie).ok_or(Error::Unauthorized)?;
            Ok(token)
        })
    }
}

pub struct AuthMiddleware {
    permission: Permission,
    auth_url: String,
}

impl AuthMiddleware {
    pub fn new(auth_url: String, permission: Permission) -> Self {
        AuthMiddleware {
            auth_url,
            permission,
        }
    }
}

pub struct InnerAuthMiddleware<S> {
    permission: Permission,
    auth_url: String,
    service: S,
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

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

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
        let response = HttpResponse::TemporaryRedirect()
            .insert_header(("location", self.auth_url.clone()))
            .finish()
            .map_into_right_body();

        if req.path().contains("oidc") {
            let res = self.service.call(req);

            return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
        }

        if let Some(mut token) = extract_token(req.cookie("token")) {
            match self.permission {
                Permission::Read => {
                    if !matches!(
                        token.active_permission,
                        Permission::Read | Permission::ReadWrite
                    ) {
                        let (request, _pl) = req.into_parts();
                        return Box::pin(
                            async move { Ok(ServiceResponse::new(request, response)) },
                        );
                    }
                }
                Permission::ReadWrite => {
                    if !matches!(token.active_permission, Permission::ReadWrite) {
                        let (request, _pl) = req.into_parts();
                        return Box::pin(
                            async move { Ok(ServiceResponse::new(request, response)) },
                        );
                    }
                }
            }

            req.extensions_mut().insert(token.active_club.clone());

            let res = self.service.call(req);

            Box::pin(async move {
                token.exp = get_current_timestamp() + 7200;
                let cookie = token.cookie().unwrap();
                let mut res = res.await.unwrap();
                res.response_mut().add_cookie(&cookie).unwrap();

                Ok(res.map_into_left_body())
            })
        } else {
            let (request, _pl) = req.into_parts();
            Box::pin(async move { Ok(ServiceResponse::new(request, response)) })
        }
    }
}

fn extract_token(cookie: Option<Cookie>) -> Option<Token> {
    let token = cookie?;

    let secret = DecodingKey::from_secret(env::var("APP_SECRET").ok()?.as_bytes());

    decode::<Token>(token.value(), &secret, &Validation::new(Algorithm::HS256))
        .ok()
        .map(|x| x.claims)
}
