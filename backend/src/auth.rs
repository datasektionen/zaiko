use std::env;

use actix_identity::Identity;
use actix_web::{
    get,
    web::{self, Data},
    HttpMessage, HttpRequest, HttpResponse,
};
use openidconnect::{
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreAuthenticationFlow, CoreClient, CoreErrorResponseType,
        CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreRevocableToken, CoreTokenType,
    },
    reqwest, AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    EmptyAdditionalClaims, EmptyExtraTokenFields, EndpointMaybeSet, EndpointNotSet, EndpointSet,
    IdTokenFields, IssuerUrl, Nonce, OAuth2TokenResponse, RedirectUrl, RevocationErrorResponseType,
    StandardErrorResponse, StandardTokenIntrospectionResponse, StandardTokenResponse,
    TokenResponse,
};
use serde::Deserialize;

#[derive(Clone)]
pub struct OIDCData {
    client: openidconnect::Client<
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
    >,
    http_client: reqwest::Client,
    nonce: Nonce,
}

#[derive(Deserialize)]
struct Query {
    code: String,
    state: String,
}

pub async fn check_auth(id: Option<Identity>, club: &String) -> bool {
    if let Some(id) = id {
        true
    } else {
        false
    }
}

pub async fn get_oidc() -> (OIDCData, String) {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(String::from("https://sso.datasektionen.se/op")).unwrap(),
        &http_client,
    )
    .await
    .unwrap();

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(env::var("OIDC_ID").unwrap()),
        Some(ClientSecret::new(env::var("OIDC_SECRET").unwrap())),
    )
    .set_redirect_uri(
        RedirectUrl::new(String::from("http://localhost:8080/oidc/callback")).unwrap(),
    );

    // let (pkce_challange, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // .set_pkce_challenge(pkce_challange)
        .url();

    let oidc = OIDCData {
        client,
        http_client,
        nonce,
    };

    (oidc, auth_url.to_string())
}

#[get("oidc/callback")]
pub async fn auth_callback(
    req: HttpRequest,
    query: web::Query<Query>,
    oidc: web::Data<OIDCData>,
) -> HttpResponse {
    let token_respones = oidc
        .client
        .exchange_code(AuthorizationCode::new(query.code.to_string()))
        .unwrap()
        // .set_pkce_verifier(oidc.pkce_verifier)
        .request_async(&oidc.http_client)
        .await
        .unwrap();

    let id_token = token_respones.id_token().unwrap();

    let id_token_verifier = oidc.client.id_token_verifier();
    let claims = id_token.claims(&id_token_verifier, &oidc.nonce).unwrap();

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_respones.access_token(),
            id_token.signing_alg().unwrap(),
            id_token.signing_key(&id_token_verifier).unwrap(),
        )
        .unwrap();
        if actual_access_token_hash != *expected_access_token_hash {
            panic!()
        }
    }

    Identity::login(&req.extensions(), claims.subject().to_string()).unwrap();

    println!("{:?}", claims);
    HttpResponse::TemporaryRedirect()
        .insert_header(("location", "/"))
        .finish()
}
