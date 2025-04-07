use std::{collections::HashMap, env, future::Future, pin::Pin};

use actix_web::{
    cookie::{Cookie, SameSite},
    FromRequest, HttpRequest,
};
use derive_more::Display;
use jsonwebtoken::{decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use openidconnect::{
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreAuthenticationFlow, CoreErrorResponseType,
        CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreRevocableToken, CoreTokenType,
    },
    reqwest, Client, ClientId, ClientSecret, CsrfToken, EmptyAdditionalClaims,
    EmptyExtraTokenFields, EndpointMaybeSet, EndpointNotSet, EndpointSet, IdTokenFields, IssuerUrl,
    Nonce, RedirectUrl, RevocationErrorResponseType, StandardErrorResponse,
    StandardTokenIntrospectionResponse, StandardTokenResponse,
};

use crate::error::Error;
use serde::{Deserialize, Serialize};

pub type OIDCClient = openidconnect::Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    AuthTokenResponse,
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

pub type AuthTokenResponse = StandardTokenResponse<
    IdTokenFields<
        EmptyAdditionalClaims,
        EmptyExtraTokenFields,
        CoreGenderClaim,
        CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm,
    >,
    CoreTokenType,
>;

pub type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

#[derive(Clone)]
pub(crate) struct OIDCData {
    pub(crate) client: OIDCClient,
    pub(crate) http_client: reqwest::Client,
    // pkce_verifier: PkceCodeVerifier,
    pub(crate) nonce: Nonce,
    pub(crate) csrf_token: CsrfToken,
}

impl OIDCData {
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
}

#[derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "r")]
    Read,
    #[serde(rename = "rw")]
    ReadWrite,
}

impl Permission {
    pub async fn get_pls_permissions(
        subject: String,
    ) -> Result<HashMap<String, Permission>, Error> {
        let pls_url = env::var("PLS_URL")?;

        let res = reqwest::get(format!("{}/user/{}/zaiko", pls_url, subject.as_str()))
            .await?
            .text()
            .await?;

        let mut privlages: HashMap<String, Permission> = Permission::default_privlages();

        let pls_permissions = serde_json::from_str::<Vec<String>>(&res)?;

        for privlage in pls_permissions {
            let mut permission = privlage.split("-");
            let name = permission
                .next()
                .ok_or(Error::InternalServerError(String::from(
                    "Incorrect formating och pls permission",
                )))?
                .to_string();
            let permission =
                match permission
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
            privlages.insert(name, permission);
        }

        Ok(privlages)
    }

    pub fn default_privlages() -> HashMap<String, Permission> {
        let mut privlages = HashMap::new();

        privlages.insert(String::from("metadorerna"), Permission::Read);

        privlages
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub sub: String,
    pub exp: u64,
    pub permissions: HashMap<String, Permission>,
    pub active_club: String,
    pub active_permission: Permission,
}

impl Token {
    pub fn new(sub: String, permissions: HashMap<String, Permission>) -> Option<Self> {
        let active: Club = permissions.iter().next()?.into();

        Some(Token {
            sub,
            exp: get_current_timestamp() + 7200,
            permissions,
            active_club: active.name,
            active_permission: active.permission,
        })
    }

    pub fn cookie(&self) -> Result<Cookie, Error> {
        let secret = EncodingKey::from_secret(env::var("APP_SECRET")?.as_bytes());

        let token = encode(&Header::default(), &self, &secret).unwrap();

        Ok(Cookie::build("token", token)
            .same_site(SameSite::Lax)
            .secure(true)
            .http_only(true)
            .path("/")
            .finish())
    }

    pub fn set_active_club(&mut self, club: String) -> Result<(), Error> {
        let permission = self.permissions.get(&club).ok_or(Error::Unauthorized)?;
        self.active_club = club;
        self.active_permission = permission.clone();
        Ok(())
    }

    pub fn extract_token(cookie: Option<Cookie>) -> Option<Token> {
        let token = cookie?;

        let secret = DecodingKey::from_secret(env::var("APP_SECRET").ok()?.as_bytes());

        decode::<Token>(token.value(), &secret, &Validation::new(Algorithm::HS256))
            .ok()
            .map(|x| x.claims)
    }
}

impl FromRequest for Token {
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let cookie = req.cookie("token");
        Box::pin(async move {
            let token = Token::extract_token(cookie).ok_or(Error::Unauthorized)?;
            Ok(token)
        })
    }
}

pub struct AuthMiddleware {
    pub permission: Permission,
    pub auth_url: String,
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
    pub permission: Permission,
    pub auth_url: String,
    pub service: S,
}
