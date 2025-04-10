use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::error::Error;

#[derive(Debug, Serialize)]
struct Log {
    amount: f32,
    time: i32,
}

#[get("/log")]
pub(crate) async fn get_log(
    item: web::Query<i32>,
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

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
