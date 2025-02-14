use std::env;

use actix_cors::Cors;
use actix_identity::{IdentityExt, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    guard::Guard,
    http::Method,
    web::{self, scope, Data},
    App, HttpServer,
};
use auth::{auth_callback, get_clubs, get_oidc};
use dotenv::dotenv;
use sqlx::SqlitePool;
use supplier::get_suppliers;

mod auth;
mod error;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    if env::var("APP_ENV") == Ok(String::from("development")) {
        dotenv().expect(".env to exist");
    }

    let pool = web::Data::new(
        SqlitePool::connect(&env::var("DATABASE_PATH").expect("DATABASE_PATH in .env"))
            .await
            .expect("Expected sqlite database with name db.sqlite"),
    );

    let (oidc, auth_path) = get_oidc().await;
    let oidc = Data::new(oidc);
    let auth_url = Data::new(auth_path.clone());

    let session_secret = Key::from(
        env::var("APP_SECRET")
            .expect("APP_SECRET in .env")
            .as_bytes(),
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec![
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::DELETE,
            ])
            .allow_any_header();

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), session_secret.clone())
                    .cookie_name(String::from("session"))
                    .cookie_secure(true)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::minutes(15)),
                    )
                    .build(),
            )
            .wrap(cors)
            .app_data(pool.clone())
            .app_data(oidc.clone())
            .app_data(auth_url.clone())
            .service(
                scope("/api")
                    .service(get_item)
                    .service(add_item)
                    .service(update_item)
                    .service(delete_item)
                    .service(get_supplier)
                    .service(get_suppliers)
                    .service(add_supplier)
                    .service(update_supplier)
                    .service(delete_supplier)
                    .service(get_shortage)
                    .service(take_stock)
                    .service(get_log)
                    .service(auth_callback)
                    .service(get_clubs),
            )
            .service(serve_frontend)
            .service(
                actix_files::Files::new("/", "../dist/")
                    .index_file("index.html")
                    .guard(LoginGuard),
            )
            .service(web::redirect("/", auth_path.to_string()))
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

struct LoginGuard;

impl Guard for LoginGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        ctx.get_identity().is_ok()
    }
}
