use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{error::Error, item::ItemGetResponse};

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

#[get("/stock")]
pub(crate) async fn get_shortage(
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

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

#[post("/stock")]
pub(crate) async fn take_stock(
    pool: web::Data<Pool<Postgres>>,
    body: String,
    club: web::ReqData<String>
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

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
