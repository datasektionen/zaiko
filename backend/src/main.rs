use actix_cors::Cors;
use actix_identity::{IdentityExt, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    guard::Guard,
    web::{self, scope, Data},
    App, HttpServer,
};
use auth::{auth_callback, get_clubs, get_oidc};
use dotenv::dotenv;
use sqlx::SqlitePool;
use supplier::get_suppliers;

mod auth;
mod item;
mod log;
mod serve;
mod shortage;
mod supplier;
mod error;

use crate::item::{add_item, delete_item, get_item, update_item};
use crate::log::get_log;
use crate::serve::serve_frontend;
use crate::shortage::{get_shortage, take_stock};
use crate::supplier::{add_supplier, delete_supplier, get_supplier, update_supplier};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().expect(".env to exist");

    let pool = web::Data::new(
        SqlitePool::connect("db.sqlite")
            .await
            .expect("Expected sqlite database with name db.sqlite"),
    );

    let (oidc, auth_path) = get_oidc().await;
    let oidc = Data::new(oidc);
    let auth_url = Data::new(auth_path.clone());

    let session_secret = Key::generate();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), session_secret.clone())
                    .cookie_name(String::from("session"))
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::minutes(5)),
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
                    .service(get_clubs)
            )
            .service(auth_callback)
            .service(serve_frontend)
            .service(
                actix_files::Files::new("/", "../dist/")
                    .index_file("index.html")
                    .guard(LoginGuard),
            )
            .service(web::redirect("/", auth_path.to_string()))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

struct LoginGuard;

impl Guard for LoginGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        ctx.get_identity().is_ok()
    }
}
