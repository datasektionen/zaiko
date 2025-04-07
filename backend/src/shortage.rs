use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::error::Error;

#[derive(Serialize)]
struct ShortageGetResponse {
    id: i32,
    name: String,
    location: String,
    min: f32,
    current: f32,
    order: f32,
    supplier: Option<String>,
    link: Option<String>,
}

#[derive(Deserialize)]
struct ShortageItem {
    id: i32,
    name: String,
    location: String,
    min: Option<f32>,
    max: Option<f32>,
    link: Option<String>,
    current: f32,
    supplier: Option<String>,
}

#[derive(Deserialize)]
struct StockUpdateRequest {
    items: Vec<(i32, f32)>,
}

#[get("/stock")]
pub(crate) async fn get_shortage(
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>,
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    let items = sqlx::query_as!(
        ShortageItem,
        "SELECT items.id, items.name, location, min, max, current, items.link, suppliers.name as supplier
         FROM items 
         LEFT JOIN suppliers ON items.supplier=suppliers.id
         WHERE current <= min AND items.club = $1",
        club
    )
    .fetch_all(&mut *pool)
    .await?;

    let items: Vec<ShortageGetResponse> = items
        .iter()
        .filter_map(|item| {
            Some(ShortageGetResponse {
                id: item.id,
                name: item.name.clone(),
                location: item.location.clone(),
                current: item.current,
                order: item.max? - item.current,
                min: item.min?,
                supplier: item.supplier.clone(),
                link: item.link.clone(),
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
    club: web::ReqData<String>,
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
