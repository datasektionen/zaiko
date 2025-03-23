use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{auth::{check_auth, Permission}, error::Error, item::ItemGetResponse};

#[derive(Serialize)]
struct ShortageItem {
    id: i32,
    name: String,
    location: String,
    min: f32,
    current: f32,
    order: f32,
}

#[derive(Deserialize)]
struct StockUpdateRequest {
    items: Vec<(i32, f32)>,
}

#[get("/{club}/stock")]
pub(crate) async fn get_shortage(
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let club = club.as_ref();
    let mut pool = pool.get_ref().begin().await?;

    check_auth(&id, &session, club, Permission::Read)?;

    let items = sqlx::query_as!(
        ItemGetResponse,
        "SELECT id, name, location, min, max, current, link, supplier, updated 
         FROM items 
         WHERE current <= min AND club = $1",
        club
    )
    .fetch_all(&mut *pool)
    .await?;

    let items: Vec<ShortageItem> = items
        .iter()
        .filter_map(|item| {
            Some(ShortageItem {
                id: item.id,
                name: item.name.clone(),
                location: item.location.clone(),
                current: item.current,
                order: item.max? - item.current,
                min: item.min?,
            })
        })
        .collect();

    pool.commit().await?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("/{club}/stock")]
pub(crate) async fn take_stock(
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
    body: String,
) -> Result<HttpResponse, Error> {
    let club = club.as_ref();
    let mut pool = pool.get_ref().begin().await?;

    check_auth(&id, &session, club, Permission::ReadWrite)?;

    let items: StockUpdateRequest = serde_json::from_str(&body)?;

    for (id, amount) in items.items {
        sqlx::query!(
            "UPDATE items 
             SET current = $1 
             WHERE id = $2 AND club = $3",
            amount,
            id,
            club
        )
        .execute(&mut *pool)
        .await?;

        sqlx::query!(
            "INSERT INTO log (item_id, amount, time, club) 
             VALUES ($1, $2, extract(epoch from now()), $3)",
            id,
            amount,
            club
        )
        .execute(&mut *pool)
        .await?;
    }

    pool.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
