use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use utoipa::IntoParams;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    db::{self, log::Log},
    error::Error,
};

/// Info used to filter the logs to return
#[derive(Deserialize, IntoParams)]
struct LogGetQuery {
    /// The name of the item
    name: String,
}

pub(crate) fn config() -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut ServiceConfig| {
        cfg.service(get_log);
    }
}

#[utoipa::path(
    tag = "log",
    params(LogGetQuery),
    responses(
        (
            status = StatusCode::OK,
            body = Vec<Log>,
            description = "List of logs for an item"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[get("/log")]
pub(crate) async fn get_log(
    query: web::Query<LogGetQuery>,
    db: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let logs: Vec<Log> = db::log::get_all_by_item(&db, &query.name).await?;

    Ok(HttpResponse::Ok().json(logs))
}
