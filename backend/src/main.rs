use std::env;

use actix_cors::Cors;
use actix_web::{
    http::Method,
    middleware::Logger,
    web::{self, scope, Data},
    App, HttpServer,
};
use auth::types::{AuthMiddleware, Permission};
use auth::{auth_callback, get_clubs, set_club, types::OIDCData};
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
        let _ = dotenv();
    }

    let pool = web::Data::new(
        PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL to exist"))
            .await
            .expect("Expected to connect to database"),
    );

    db_init(&pool).await.expect("to setup db");

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
                    Method::DELETE,
                ])
                .allow_any_header()
                .block_on_origin_mismatch(true)
        };

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(AuthMiddleware::new(auth_url.clone(), Permission::Read))
            .app_data(pool.clone())
            .app_data(oidc.clone())
            .service(auth_callback)
            .service(
                scope("/api")
                    .service(get_item)
                    .service(get_shortage)
                    .service(get_log)
                    .service(get_stats)
                    .service(get_clubs)
                    .service(set_club)
                    .service(
                        scope("/admin")
                            .wrap(AuthMiddleware::new(auth_url.clone(), Permission::ReadWrite))
                            .service(get_supplier)
                            .service(get_suppliers)
                            .service(add_item)
                            .service(update_item)
                            .service(delete_item)
                            .service(add_supplier)
                            .service(update_supplier)
                            .service(delete_supplier)
                            .service(take_stock),
                    ),
            )
            .service(serve_frontend)
            .service(actix_files::Files::new("/", "../dist/").index_file("index.html"))
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
