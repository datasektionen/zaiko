#[warn(clippy::pedantic)]
use actix_cors::Cors;
use actix_web::{
    http::Method,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_actix_web::{scope, AppExt};
use utoipa_redoc::{Redoc, Servable};

mod auth;
mod db;
mod error;
mod item;
mod logging;
mod serve;
mod shipment;
mod shortage;
mod stats;
mod storage;
mod supplier;

use auth::types::{AuthMiddleware, OIDCData};
use db::init_db;
use serve::serve_frontend;

#[derive(OpenApi)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    if env::var("APP_ENV") == Ok(String::from("development")) {
        let _ = dotenv();
    }

    let pool = web::Data::new(init_db().await);

    sqlx::migrate!("./migrations")
        .run(pool.get_ref())
        .await
        .expect("migrations to run");

    let (oidc, auth_path) = OIDCData::get_oidc().await;
    let oidc = Data::new(oidc);
    let auth_url = auth_path.clone();

    HttpServer::new(move || {
        let cors = if env::var("APP_ENV") == Ok(String::from("development")) {
            Cors::permissive()
        } else {
            Cors::default()
                .allowed_methods(vec![
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::PUT,
                    Method::DELETE,
                ])
                .allow_any_header()
                .allowed_origin("https://sso.datasektionen.se")
        };

        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| {
                app.wrap(cors)
                    .wrap(Logger::default())
                    .app_data(pool.clone())
                    .app_data(oidc.clone())
            })
            .service(utoipa_actix_web::scope("/auth").configure(auth::config()))
            .service(
                scope("/api") // Every thing in this scope requires the user to be logged in using sso
                    .wrap(AuthMiddleware::new(auth_url.clone()))
                    .configure(item::config())
                    .configure(supplier::config())
                    .configure(storage::config())
                    .configure(shortage::config())
                    .configure(stats::config())
                    .configure(logging::config())
                    .configure(shipment::config())
                    .service(auth::user_info),
            )
            .openapi_service(|api| Redoc::with_url("/docs/api", api))
            .service(scope("").map(|app| {
                app
                    .service(actix_files::Files::new("/assets/", "dist/assets/").index_file("index.html"))
                    .service(actix_files::Files::new("/static/", "dist/static/").index_file("index.html"))
                    .service(serve_frontend)
                    .service(actix_files::Files::new("/", "dist/").index_file("index.html"))
            }))
            .into_app()
    })
    .bind((
        env::var("APP_URL").expect("APP_URL in .env"),
        env::var("PORT")
            .expect("PORT in .env")
            .parse()
            .expect("PORT to be a number"),
    ))?
    .run()
    .await
}
