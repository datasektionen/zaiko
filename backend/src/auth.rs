use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use derive_more::Display;
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
use std::{collections::HashMap, env};

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

#[derive(Debug, Serialize, Deserialize)]
struct Club {
    name: String,
    permission: Permission,
}

#[derive(Deserialize)]
struct Query {
    code: String,
    state: String,
}

pub fn check_auth(
    session: &Session,
    club: &String,
    required_permission: Permission,
) -> Result<(), Error> {
    match get_permissions(session, club) {
        Some(Permission::ReadWrite) => Ok(()),
        Some(Permission::Read) => match required_permission {
            Permission::Read => Ok(()),
            Permission::ReadWrite => Err(Error::Unauthorized),
        },
        None => Err(Error::Unauthorized),
    }
}

pub fn get_permissions(session: &Session, club: &String) -> Option<Permission> {
    Some(
        session
            .get::<HashMap<String, Permission>>("privlages")
            .ok()??
            .get(club)?
            .clone(),
    )
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

#[get("/oidc/callback")]
pub async fn auth_callback(
    req: HttpRequest,
    session: Session,
    query: web::Query<Query>,
    oidc: web::Data<OIDCData>,
) -> Result<HttpResponse, Error> {
    let Query { code, state } = query.0;
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

    Identity::login(&req.extensions(), claims.subject().to_string())?;

    let privlages = get_pls_permissions(claims.subject().to_string()).await?;

    session.insert("privlages", privlages)?;

    Ok(HttpResponse::TemporaryRedirect()
        .insert_header(("location", "/"))
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

    log::debug!("{}", pls_url);
    log::debug!("{}", subject.as_str());

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
pub async fn get_clubs(session: Session) -> Result<HttpResponse, Error> {
    if env::var("APP_AUTH") == Ok(String::from("false")) {
        log::warn!("fake auth");
        let mut clubs = HashMap::new();
        clubs.insert(String::from("metadorerna"), Permission::ReadWrite);
        session.insert("privlages", clubs)?;

        let clubs = vec![Club {
            name: String::from("metadorerna"),
            permission: Permission::ReadWrite,
        }];
        return Ok(HttpResponse::Ok().json(clubs));
    }

    let clubs: Vec<Club> = session
        .get::<HashMap<String, Permission>>("privlages")?
        .ok_or(Error::InternalServerError(String::from(
            "Session contians no permissions",
        )))?
        .into_iter()
        .map(|(name, permission)| Club { name, permission })
        .collect();

    Ok(HttpResponse::Ok().json(clubs))
}
