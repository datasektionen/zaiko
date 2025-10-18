use actix_web::{
    get,
    web::{self},
    HttpResponse,
};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{db, error::Error};

/// Genaral info about the state of all storages
#[derive(Debug, Serialize, ToSchema)]
struct StatsGetResponse {
    /// The total number of items
    items: i64,
    /// The total number of suppliers
    suppliers: i64,
    /// The total number of shortages
    shortages: i64,
}

pub(crate) fn config() -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut ServiceConfig| {
        cfg.service(get_stats);
    }
}

#[utoipa::path(
    tag = "stats",
    responses(
        (
            status = StatusCode::OK,
            body = StatsGetResponse,
            description = "General info about the entire organisation"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[get("/stats")]
async fn get_stats(db: web::Data<Pool<Postgres>>) -> Result<HttpResponse, Error> {
    let item_count = db::item::get_count(&db).await?.unwrap_or(0);
    let supplier_count = db::supplier::get_count(&db).await?.unwrap_or(0);
    let shortage_count = db::item::get_shortage_count(&db).await?.unwrap_or(0);

    let stats = StatsGetResponse {
        items: item_count,
        suppliers: supplier_count,
        shortages: shortage_count,
    };

    Ok(HttpResponse::Ok().json(stats))
}
