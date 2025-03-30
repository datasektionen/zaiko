use std::{env, time::Duration};

use actix_cors::Cors;
use actix_identity::{IdentityExt, IdentityMiddleware};
use actix_session::{
    config::BrowserSession, storage::CookieSessionStore, SessionExt, SessionMiddleware,
};
use actix_web::{
    cookie::Key,
    guard::Guard,
    http::Method,
    middleware::Logger,
    web::{self, scope, Data},
    App, HttpServer,
};
use auth::{auth_callback, check_auth, get_clubs, get_oidc, Permission};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use stats::get_stats;
use supplier::get_suppliers;

mod auth;
mod error;
mod item;
mod loging;
mod serve;
mod shortage;
mod stats;
mod supplier;

use crate::item::{add_item, delete_item, get_item, update_item};
use crate::loging::get_log;
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
        PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL to exist"))
            .await
            .expect("Expected to connect to database"),
    );

    db_init(&pool).await.expect("to setup db");

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
            .allow_any_header()
            .block_on_origin_mismatch(true);

        App::new()
            .wrap(
                IdentityMiddleware::builder()
                    .visit_deadline(Some(Duration::from_secs(3600)))
                    .build(),
            )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), session_secret.clone())
                    .cookie_name(String::from("session"))
                    .cookie_secure(true)
                    .session_lifecycle(BrowserSession::default())
                    .build(),
            )
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(pool.clone())
            .app_data(oidc.clone())
            .app_data(auth_url.clone())
            .service(
                // Enpoints requiring write access (also needs to contian enpoints requiring read
                // acccess)
                scope("/api")
                    .guard(WriteGuard)
                    .service(get_item)
                    .service(get_supplier)
                    .service(get_suppliers)
                    .service(get_shortage)
                    .service(get_log)
                    .service(get_stats)
                    .service(add_item)
                    .service(update_item)
                    .service(delete_item)
                    .service(add_supplier)
                    .service(update_supplier)
                    .service(delete_supplier)
                    .service(take_stock),
            )
            .service(
                // Endpoints requiring read access
                scope("/api")
                    .guard(ReadGuard)
                    .service(get_item)
                    .service(get_supplier)
                    .service(get_suppliers)
                    .service(get_shortage)
                    .service(get_log)
                    .service(get_stats),
            )
            .service(
                // Endpoints requiring no access
                scope("/api").service(auth_callback).service(get_clubs),
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
struct ReadGuard;
struct WriteGuard;

impl Guard for LoginGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        ctx.get_identity().is_ok()
    }
}

impl Guard for ReadGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        log::debug!("read guard");
        if ctx.get_identity().is_err() {
            log::debug!("no id");
            return false;
        }

        let session = ctx.get_session();

        if let Some(club) = ctx.head().uri.path().split("/").nth(2) {
            log::debug!("path: {}", club);
            check_auth(&session, &club.to_string(), Permission::Read).is_ok()
        } else {
            log::debug!("no path");
            false
        }
    }
}

impl Guard for WriteGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        log::debug!("write guard");
        if ctx.get_identity().is_err() {
            log::debug!("no id");
            return false;
        }

        let session = ctx.get_session();

        if let Some(club) = ctx.head().uri.path().split("/").nth(2) {
            log::debug!("path: {}", club);
            check_auth(&session, &club.to_string(), Permission::ReadWrite).is_ok()
        } else {
            log::debug!("no path");
            false
        }
    }
}

async fn db_init(pool: &web::Data<Pool<Postgres>>) -> Result<(), sqlx::error::Error> {
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS items(
        id SERIAL PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        location TEXT NOT NULL,
        min REAL,
        max REAL,
        current REAL NOT NULL,
        supplier INTEGER,
        updated INTEGER NOT NULL,
        link TEXT,
        club TEXT NOT NULL);",
    )
    .execute(pool.get_ref())
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS suppliers(
        id SERIAL PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        link TEXT,
        notes TEXT,
        username TEXT,
        password TEXT,
        updated INTEGER NOT NULL,
        club TEXT NOT NULL);"
    )
    .execute(pool.get_ref())
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS log(
        id SERIAL PRIMARY KEY NOT NULL,
        item_id INTEGER NOT NULL,
        amount REAL NOT NULL,
        time INTEGER NOT NULL,
        club TEXT NOT NULL);"
    )
    .execute(pool.get_ref())
    .await?;

    Ok(())
}
