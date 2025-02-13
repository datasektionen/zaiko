use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

use crate::auth::check_auth;

#[derive(Debug, Serialize)]
struct Log {
    amount: f64,
    time: i64,
}

#[get("/{club}/log")]
pub(crate) async fn get_log(
    club: web::Path<String>,
    item: web::Query<i64>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    let club = club.as_ref();

    if !check_auth(id, session, club).await {
        return HttpResponse::Unauthorized().finish()
    } 

    match sqlx::query_as!(
        Log,
        "SELECT amount, time FROM log WHERE item_id = $1 AND club = $2",
        item.0,
        club
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
