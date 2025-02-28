use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::{auth::check_auth, error::Error};

#[derive(Debug, Serialize)]
struct StatsGetResponse {
    items: i64,
    suppliers: i64,
    shortages: i64,
}

#[get("/{club}/stats")]
pub(crate) async fn get_stats(
    club: web::Path<String>,
    pool: web::Data<Pool<Postgres>>,
    id: Option<Identity>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let club = club.as_ref();
    let mut pool = pool.as_ref().begin().await?;

    check_auth(&id, &session, club).await?;

    let item_count = sqlx::query!(
        "SELECT count(*) 
         FROM items 
         WHERE club = $1",
        club
    )
    .fetch_one(&mut *pool)
    .await?
    .count
    .unwrap_or(0);

    let supplier_count = sqlx::query!(
        "SELECT count(*) 
         FROM suppliers 
         WHERE club = $1",
        club
    )
    .fetch_one(&mut *pool)
    .await?
    .count
    .unwrap_or(0);

    let shortage_count = sqlx::query!(
        "SELECT count(*) 
         FROM items 
         WHERE current <= min AND club = $1",
        club
    )
    .fetch_one(&mut *pool)
    .await?
    .count
    .unwrap_or(0);

    let stats = StatsGetResponse {
        items: item_count,
        suppliers: supplier_count,
        shortages: shortage_count,
    };

    pool.commit().await?;

    Ok(HttpResponse::Ok().json(stats))
}
