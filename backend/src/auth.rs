use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use openidconnect::{
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreAuthenticationFlow, CoreErrorResponseType,
        CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreRevocableToken, CoreTokenType,
    },
    reqwest, AccessTokenHash, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken,
    EmptyAdditionalClaims, EmptyExtraTokenFields, EndpointMaybeSet, EndpointNotSet, EndpointSet,
    IdTokenFields, IssuerUrl, Nonce, OAuth2TokenResponse, RedirectUrl,
    RevocationErrorResponseType, StandardErrorResponse, StandardTokenIntrospectionResponse,
    StandardTokenResponse, TokenResponse,
};
use serde::Deserialize;
use std::env;

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

#[derive(Deserialize)]
struct Query {
    code: String,
    state: String,
}

pub async fn check_auth(id: Option<Identity>, session: Session, club: &String) -> bool {
    if id.is_some() {
        if let Ok(Some(privlages)) = session.get::<Vec<String>>("privlages") {
            return privlages.contains(club);
        }
    }

    false
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

#[get("oidc/callback")]
pub async fn auth_callback(
    req: HttpRequest,
    session: Session,
    query: web::Query<Query>,
    oidc: web::Data<OIDCData>,
) -> HttpResponse {
    if *oidc.csrf_token.secret() != query.state {
        log::error!("Invalid CSRF token on oidc callback");
        return HttpResponse::BadRequest().finish();
    }

    let token_respones = match oidc
        .client
        .exchange_code(AuthorizationCode::new(query.code.to_string()))
    {
        Ok(request) => match request.request_async(&oidc.http_client).await {
            Ok(token) => token,
            Err(err) => {
                log::error!("{}", err);
                return HttpResponse::InternalServerError().finish();
            }
        },
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let id_token = match token_respones.id_token() {
        Some(token) => token,
        None => {
            log::error!("oidc server returned no id");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let id_token_verifier = oidc.client.id_token_verifier();

    let claims = match id_token.claims(&id_token_verifier, &oidc.nonce) {
        Ok(claims) => claims,
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = match AccessTokenHash::from_token(
            token_respones.access_token(),
            match id_token.signing_alg() {
                Ok(alg) => alg,
                Err(err) => {
                    log::error!("{}", err);
                    return HttpResponse::InternalServerError().finish();
                }
            },
            match id_token.signing_key(&id_token_verifier) {
                Ok(key) => key,
                Err(err) => {
                    log::error!("{}", err);
                    return HttpResponse::InternalServerError().finish();
                }
            },
        ) {
            Ok(hash) => hash,
            Err(err) => {
                log::error!("{}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
        if actual_access_token_hash != *expected_access_token_hash {
            log::error!(
                "Hashes did not mach for subject {}",
                claims.subject().to_string()
            );
            return HttpResponse::InternalServerError().finish();
        }
    }

    if let Err(err) = Identity::login(&req.extensions(), claims.subject().to_string()) {
        log::error!("{}", err);
        return HttpResponse::InternalServerError().finish();
    }

    let res = match reqwest::get(format!(
        "https://pls.datasektionen.se/api/user/{}/zaiko",
        claims.subject().as_str()
    ))
    .await
    {
        Ok(res) => match res.text().await {
            Ok(res) => res,
            Err(err) => {
                log::error!("{}", err);
                return HttpResponse::InternalServerError().finish();
            }
        },
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let privlages: Vec<String> = match serde_json::from_str(&res) {
        Ok(privlages) => privlages,
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Err(err) = session.insert("privlages", privlages) {
        log::error!("{}", err);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::TemporaryRedirect()
        .insert_header(("location", "/"))
        .finish()
}
