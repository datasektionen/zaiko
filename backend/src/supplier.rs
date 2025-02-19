use actix_identity::Identity;
use actix_session::Session;
use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::{auth::check_auth, error::Error};

#[derive(Debug, Serialize, Deserialize)]
struct SupplierGetResponse {
    id: i64,
    name: String,
    link: Option<String>,
    notes: Option<String>,
    username: Option<String>,
    password: Option<String>,
    updated: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SupplierListGetResponse {
    id: i64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SupplierAddRequest {
    name: String,
    link: Option<String>,
    notes: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SupplierUpdateRequest {
    id: i64,
    name: String,
    link: Option<String>,
    notes: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Deserialize)]
struct Query {
    id: Option<i64>,
}

#[get("/{club}/supplier")]
pub(crate) async fn get_supplier(
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
    query: web::Query<Query>,
) -> Result<HttpResponse, Error> {
    log::info!("get supplier");

    let club = club.as_ref();

    check_auth(id, session, club).await?;

    if let Some(id) = query.id {
        let name = sqlx::query!("SELECT name FROM suppliers WHERE id = $1", id)
            .fetch_one(pool.get_ref())
            .await?
            .name;

        Ok(HttpResponse::Ok().json(name))
    } else {
        let suppliers = sqlx::query_as!(
            SupplierGetResponse,
            "SELECT id, name, username, password, link, notes, updated FROM suppliers WHERE club = $1",
            club
        )
        .fetch_all(pool.get_ref())
        .await?;

        Ok(HttpResponse::Ok().json(suppliers))
    }
}

#[get("/{club}/suppliers")]
pub(crate) async fn get_suppliers(
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, Error> {
    log::info!("get suppliers");

    let club = club.as_ref();

    check_auth(id, session, club).await?;

    let supplier = sqlx::query_as!(
        SupplierListGetResponse,
        "SELECT id, name FROM suppliers WHERE club = $1",
        club
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(supplier))
}

#[post("/{club}/supplier")]
pub(crate) async fn add_supplier(
    body: String,
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, Error> {
    log::info!("add supplier");
    log::debug!("{}", body);

    let club = club.as_ref();

    check_auth(id, session, club).await?;

    let supplier: SupplierAddRequest = serde_json::from_str(&body)?;

    if supplier.name.is_empty() {
        return Err(Error::BadRequest);
    }

    sqlx::query!(
        "INSERT INTO suppliers (name, link, notes, username, password, updated, club) VALUES ($1, $2, $3, $4, $5, strftime('%s', 'now'), $6)",
        supplier.name,
        supplier.link,
        supplier.notes,
        supplier.username,
        supplier.password,
        club,
    )
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[patch("/{club}/supplier")]
pub(crate) async fn update_supplier(
    club: web::Path<String>,
    body: String,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, Error> {
    log::info!("update supplier");
    log::debug!("{}", body);

    let club = club.as_ref();

    check_auth(id, session, club).await?;

    let supplier: SupplierUpdateRequest = serde_json::from_str(&body)?;

    if supplier.name.is_empty() {
        return Err(Error::BadRequest);
    }

    sqlx::query!(
        "UPDATE suppliers SET name = $1, link = $2, notes = $3, username = $4, password = $5, updated = strftime('%s', 'now') WHERE id = $6 AND club = $7",
        supplier.name,
        supplier.link,
        supplier.notes,
        supplier.username,
        supplier.password,
        supplier.id,
        club,
    )
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/{club}/supplier")]
pub(crate) async fn delete_supplier(
    club: web::Path<String>,
    item_id: web::Query<i64>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, Error> {
    let club = club.as_ref();

    check_auth(id, session, club).await?;

    sqlx::query!(
        "DELETE FROM suppliers WHERE id = $1 AND club = $2",
        item_id.0,
        club
    )
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}
