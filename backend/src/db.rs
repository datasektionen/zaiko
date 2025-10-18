use std::env;

use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use utoipa::ToSchema;

pub mod container;
pub mod interval;
pub mod item;
pub mod log;
pub mod shipment;
pub mod storage;
pub mod supplier;

#[derive(Debug, PartialEq, Serialize, sqlx::Type, ToSchema)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "state")]
pub enum OrderState {
    None,
    Good,
    Warning,
    Critical,
    Incoming,
}

pub async fn init_db() -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL to exist"))
        .await
        .expect("Expected to connect to database")
}
