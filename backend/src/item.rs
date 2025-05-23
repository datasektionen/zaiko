use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ItemGetResponse {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) min: Option<f32>,
    pub(crate) max: Option<f32>,
    pub(crate) current: f32,
    pub(crate) supplier: Option<String>,
    pub(crate) updated: i32,
    pub(crate) link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ItemAddRequest {
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) min: Option<f32>,
    pub(crate) max: Option<f32>,
    pub(crate) current: f32,
    pub(crate) supplier: Option<i32>,
    pub(crate) link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ItemUpdateRequest {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) min: Option<f32>,
    pub(crate) max: Option<f32>,
    pub(crate) current: f32,
    pub(crate) supplier: Option<i32>,
    pub(crate) link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemGetQuery {
    column: Option<String>,
    search: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ItemDeleteQuery {
    id: i32,
}

#[get("/item")]
pub(crate) async fn get_item(
    pool: web::Data<Pool<Postgres>>,
    query: web::Query<ItemGetQuery>,
    club: web::ReqData<String>,
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    let items = if let ItemGetQuery {
        column: Some(column),
        search: Some(search),
    } = query.0
    {
        if matches!(column.as_str(), "name" | "location" | "link") {
            sqlx::query_as!(
                ItemGetResponse,
                r#"SELECT items.id, items.name, location, min, max, current, items.link, suppliers.name AS "supplier?", items.updated 
                 FROM items
                 LEFT JOIN suppliers ON items.supplier=suppliers.id
                 WHERE items.club = $1"#,
                club
            )
            .fetch_all(&mut *pool)
            .await?
        } else if matches!(
            column.as_str(),
            "min" | "max" | "current" | "supplier" | "updated"
        ) {
            sqlx::query_as!(
                ItemGetResponse,
                r#"SELECT items.id, items.name, location, min, max, current, items.link, suppliers.name AS "supplier?", items.updated 
                 FROM items 
                 LEFT JOIN suppliers ON items.supplier=suppliers.id
                 WHERE items.club = $1 AND $2 = $3"#,
                club,
                column,
                search
            )
            .fetch_all(&mut *pool)
            .await?
        } else {
            return Err(Error::BadRequest);
        }
    } else {
        sqlx::query_as!(
            ItemGetResponse,
            r#"SELECT items.id, items.name, location, min, max, current, items.link, suppliers.name AS "supplier?", items.updated 
             FROM items 
             LEFT JOIN suppliers ON items.supplier=suppliers.id
             WHERE items.club = $1"#,
            club
        )
        .fetch_all(&mut *pool)
        .await?
    };

    pool.commit().await?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("/item")]
pub(crate) async fn add_item(
    body: String,
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>,
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    let item: ItemAddRequest = serde_json::from_str(&body)?;

    if item.name.is_empty() || item.location.is_empty() {
        return Err(Error::BadRequest);
    }

    sqlx::query!(
        "INSERT INTO items (name, location, min, max, current, supplier, updated, link, club) 
         VALUES ($1, $2, $3, $4, $5, $6, extract(epoch from now()), $7, $8)",
        item.name,
        item.location,
        item.min,
        item.max,
        item.current,
        item.supplier,
        item.link,
        club
    )
    .execute(&mut *pool)
    .await?;

    let id = sqlx::query!(
        "SELECT id 
         FROM items 
         WHERE name = $1 AND club = $2",
        item.name,
        club
    )
    .fetch_one(&mut *pool)
    .await?
    .id;

    sqlx::query!(
        "INSERT INTO log (item_id, amount, time, club) 
         VALUES ($1, $2, extract(epoch from now()), $3)",
        id,
        item.current,
        club
    )
    .execute(&mut *pool)
    .await?;

    pool.commit().await?;

    Ok(HttpResponse::Ok().finish())
}

#[patch("/item")]
pub(crate) async fn update_item(
    body: String,
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>,
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    let item: ItemUpdateRequest = serde_json::from_str(&body)?;

    if item.name.is_empty() || item.location.is_empty() {
        return Err(Error::BadRequest);
    }

    let current = sqlx::query!(
        "SELECT current 
         FROM items 
         WHERE name = $1 AND club = $2",
        item.name,
        club
    )
    .fetch_one(&mut *pool)
    .await?
    .current;

    if current != item.current {
        sqlx::query!(
            "INSERT INTO log (item_id, amount, time, club) 
             VALUES ($1, $2, extract(epoch from now()), $3)",
            item.id,
            item.current,
            club
        )
        .execute(&mut *pool)
        .await?;
    }

    sqlx::query!(
        "UPDATE items 
         SET location = $1, min = $2, max = $3, current = $4, supplier = $5, updated = extract(epoch from now()), link = $6  WHERE name = $7 AND club = $8",
        item.location,
        item.min,
        item.max,
        item.current,
        item.supplier,
        item.link,
        item.name,
        club
    )
    .execute(&mut *pool)
    .await?;

    pool.commit().await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/item")]
pub(crate) async fn delete_item(
    item_id: web::Query<ItemDeleteQuery>,
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>,
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    sqlx::query!(
        "DELETE FROM items 
         WHERE id = $1 AND club = $2",
        item_id.id,
        club
    )
    .execute(&mut *pool)
    .await?;

    sqlx::query!(
        "DELETE FROM log 
         WHERE item_id = $1 AND club = $2",
        item_id.id,
        club
    )
    .execute(&mut *pool)
    .await?;

    pool.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
