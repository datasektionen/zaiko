use actix_identity::Identity;
use actix_session::Session;
use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{auth::check_auth, error::Error};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ItemGetResponse {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) min: Option<f32>,
    pub(crate) max: Option<f32>,
    pub(crate) current: f32,
    pub(crate) supplier: Option<i32>,
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

#[get("/{club}/item")]
pub(crate) async fn get_item(
    club: web::Path<String>,
    pool: web::Data<Pool<Postgres>>,
    id: Option<Identity>,
    session: Session,
) -> Result<HttpResponse, Error> {
    log::info!("get items");
    let club = club.as_ref();

    check_auth(id, session, club).await?;

    let items = sqlx::query_as!(
        ItemGetResponse,
        "SELECT id, name, location, min, max, current, link, supplier, updated FROM items WHERE club = $1",
        club
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("/{club}/item")]
pub(crate) async fn add_item(
    body: String,
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    log::info!("add item");
    log::debug!("{}", body);

    let club = club.as_ref();

    check_auth(id, session, club).await?;

    let item: ItemAddRequest = serde_json::from_str(&body)?;

    if item.name.is_empty() && item.location.is_empty() {
        return Err(Error::BadRequest);
    }

    let res = sqlx::query!(
        "INSERT INTO items (name, location, min, max, current, supplier, updated, link, club) VALUES ($1, $2, $3, $4, $5, $6, extract(epoch from now()), $7, $8)",
        item.name,
        item.location,
        item.min,
        item.max,
        item.current,
        item.supplier,
        item.link,
        club,
    )
    .execute(pool.get_ref())
    .await?;

    let id = sqlx::query!(
        "SELECT id FROM items WHERE name = $1 AND club = $2",
        item.name,
        club
    )
    .fetch_one(pool.get_ref())
    .await?
    .id;

    sqlx::query!(
        "INSERT INTO log (item_id, amount, time, club) VALUES ($1, $2, extract(epoch from now()), $3)",
        id,
        item.current,
        club
    )
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[patch("/{club}/item")]
pub(crate) async fn update_item(
    club: web::Path<String>,
    body: String,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    log::info!("update item");
    log::debug!("{}", body);

    let club = club.as_ref();

    check_auth(id, session, club).await?;

    let item: ItemUpdateRequest = serde_json::from_str(&body)?;

    if item.name.is_empty() && item.location.is_empty() {
        return Err(Error::BadRequest);
    }

    let current = sqlx::query!(
        "SELECT current FROM items WHERE name = $1 AND club = $2",
        item.name,
        club
    )
    .fetch_one(pool.as_ref())
    .await?
    .current;

    if current != item.current {
        sqlx::query!(
            "INSERT INTO log (item_id, amount, time, club) VALUES ($1, $2, extract(epoch from now()), $3)",
            item.id,
            item.current,
            club
        )
        .execute(pool.get_ref())
        .await?;
    }

    sqlx::query!(
        "UPDATE items SET location = $1, min = $2, max = $3, current = $4, supplier = $5, updated = extract(epoch from now()), link = $6  WHERE name = $7 AND club = $8",
        item.location,
        item.min,
        item.max,
        item.current,
        item.supplier,
        item.link,
        item.name,
        club,
    )
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/{club}/item")]
pub(crate) async fn delete_item(
    club: web::Path<String>,
    item_id: web::Query<i32>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let club = club.as_ref();

    check_auth(id, session, club).await?;

    sqlx::query!(
        "DELETE FROM items WHERE id = $1 AND club = $2",
        item_id.0,
        club
    )
    .execute(pool.get_ref())
    .await?;

    sqlx::query!(
        "DELETE FROM log WHERE item_id = $1 AND club = $2",
        item_id.0,
        club
    )
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}
