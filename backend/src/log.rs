use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::{auth::check_auth, error::Error};

#[derive(Debug, Serialize)]
struct Log {
    amount: f32,
    time: i32,
}

#[get("/{club}/log")]
pub(crate) async fn get_log(
    club: web::Path<String>,
    item: web::Query<i32>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let club = club.as_ref();
    let mut pool = pool.get_ref().begin().await?;

    check_auth(&id, &session, club).await?;

    let logs = sqlx::query_as!(
        Log,
        "SELECT amount, time 
         FROM log 
         WHERE item_id = $1 AND club = $2",
        item.0,
        club
    )
    .fetch_all(&mut *pool)
    .await?;

    pool.commit().await?;

    Ok(HttpResponse::Ok().json(logs))
}
