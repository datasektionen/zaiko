use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize)]
struct Log {
    amount: f64,
    time: i64,
}

#[get("/{club}/log")]
pub(crate) async fn get_log(
    club: web::Path<String>,
    item: web::Query<i64>,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    let club = club.as_ref();
    let item = *item;
    match sqlx::query_as!(
        Log,
        "SELECT amount, time FROM log WHERE item = $1 AND club = $2",
        item,
        club
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

