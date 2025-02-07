use std::env;

use actix_cors::Cors;
use actix_web::{
    get,
    web::{self, scope},
    App, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest, AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse,
};
use serde::Deserialize;
use sqlx::SqlitePool;

mod item;
mod log;
mod serve;
mod shortage;
mod supplier;

use crate::item::{add_item, delete_item, get_item, update_item};
use crate::log::get_log;
use crate::serve::serve_frontend;
use crate::shortage::{get_shortage, take_stock};
use crate::supplier::{add_supplier, delete_supplier, get_supplier, update_supplier};

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Config {
    database_url: String,
    oidc_id: String,
    oidc_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().unwrap();
    let pool = web::Data::new(
        SqlitePool::connect("db.sqlite")
            .await
            .expect("Expected sqlite database with name db.sqlite"),
    );

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

    println!("Browse to: {}", auth_url);

    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf);
    let code = buf.trim();

    let token_respones = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .unwrap()
        // .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await
        .unwrap();

    let id_token = token_respones.id_token().unwrap();

    let id_token_verifier = client.id_token_verifier();
    let claims = id_token.claims(&id_token_verifier, &nonce).unwrap();

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_respones.access_token(),
            id_token.signing_alg().unwrap(),
            id_token.signing_key(&id_token_verifier).unwrap(),
        ).unwrap();
        if actual_access_token_hash != *expected_access_token_hash {
            panic!()
        }
    }

    println!("{:?}", claims);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .app_data(pool.clone())
            .service(
                scope("/api")
                    .service(get_item)
                    .service(add_item)
                    .service(update_item)
                    .service(delete_item)
                    .service(get_supplier)
                    .service(add_supplier)
                    .service(update_supplier)
                    .service(delete_supplier)
                    .service(get_shortage)
                    .service(take_stock)
                    .service(get_log),
            )
            .service(serve_frontend)
            .service(actix_files::Files::new("/", "../dist/").index_file("index.html"))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
