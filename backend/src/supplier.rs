use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct SupplierGetResponse {
    id: i32,
    name: String,
    link: Option<String>,
    notes: Option<String>,
    username: Option<String>,
    password: Option<String>,
    updated: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SupplierListGetResponse {
    id: i32,
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
    id: i32,
    name: String,
    link: Option<String>,
    notes: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Deserialize)]
struct SupplierGetQuery {
    id: Option<i32>,
    column: Option<String>,
    search: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SupplierDeleteQuery {
    id: i32,
}

#[get("/supplier")]
pub(crate) async fn get_supplier(
    pool: web::Data<Pool<Postgres>>,
    query: web::Query<SupplierGetQuery>,
    club: web::ReqData<String>
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    if let Some(id) = query.id {
        let name = sqlx::query!(
            "SELECT name 
             FROM suppliers 
             WHERE club = $1 AND id = $2",
            club,
            id
        )
        .fetch_one(&mut *pool)
        .await?
        .name;

        Ok(HttpResponse::Ok().json(name))
    } else if let SupplierGetQuery {
        column: Some(column),
        search: Some(search),
        id: _,
    } = query.0
    {
        let suppliers = if matches!(
            column.as_str(),
            "name" | "username" | "password" | "link" | "notes"
        ) {
            sqlx::query_as!(
                SupplierGetResponse,
                "SELECT id, name, username, password, link, notes, updated 
                 FROM suppliers 
                 WHERE club = $1",
                club
            )
            .fetch_all(&mut *pool)
            .await?
        } else if matches!(column.as_str(), "updated") {
            sqlx::query_as!(
                SupplierGetResponse,
                "SELECT id, name, username, password, link, notes, updated 
                 FROM suppliers 
                 WHERE club = $1 AND $2 = $3",
                club,
                column,
                search
            )
            .fetch_all(&mut *pool)
            .await?
        } else {
            return Err(Error::BadRequest);
        };

        pool.commit().await?;

        Ok(HttpResponse::Ok().json(suppliers))
    } else {
        let suppliers = sqlx::query_as!(
            SupplierGetResponse,
            "SELECT id, name, username, password, link, notes, updated 
             FROM suppliers 
             WHERE club = $1",
            club
        )
        .fetch_all(&mut *pool)
        .await?;

        pool.commit().await?;

        Ok(HttpResponse::Ok().json(suppliers))
    }
}

#[get("/suppliers")]
pub(crate) async fn get_suppliers(
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    let supplier = sqlx::query_as!(
        SupplierListGetResponse,
        "SELECT id, name 
         FROM suppliers 
         WHERE club = $1",
        club
    )
    .fetch_all(&mut *pool)
    .await?;

    pool.commit().await?;

    Ok(HttpResponse::Ok().json(supplier))
}

#[post("/supplier")]
pub(crate) async fn add_supplier(
    body: String,
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    let supplier: SupplierAddRequest = serde_json::from_str(&body)?;

    if supplier.name.is_empty() {
        return Err(Error::BadRequest);
    }

    sqlx::query!(
        "INSERT INTO suppliers (name, link, notes, username, password, updated, club) 
         VALUES ($1, $2, $3, $4, $5, extract(epoch from now()), $6)",
        supplier.name,
        supplier.link,
        supplier.notes,
        supplier.username,
        supplier.password,
        club
    )
    .execute(&mut *pool)
    .await?;

    pool.commit().await?;

    Ok(HttpResponse::Ok().finish())
}

#[patch("/supplier")]
pub(crate) async fn update_supplier(
    body: String,
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    let supplier: SupplierUpdateRequest = serde_json::from_str(&body)?;

    if supplier.name.is_empty() {
        return Err(Error::BadRequest);
    }

    sqlx::query!(
        "UPDATE suppliers 
         SET name = $1, link = $2, notes = $3, username = $4, password = $5, updated = extract(epoch from now()) 
         WHERE id = $6 AND club = $7",
        supplier.name,
        supplier.link,
        supplier.notes,
        supplier.username,
        supplier.password,
        supplier.id,
        club
    )
    .execute(&mut *pool)
    .await?;

    pool.commit().await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/supplier")]
pub(crate) async fn delete_supplier(
    item_id: web::Query<SupplierDeleteQuery>,
    pool: web::Data<Pool<Postgres>>,
    club: web::ReqData<String>
) -> Result<HttpResponse, Error> {
    let club = club.as_str();
    let mut pool = pool.get_ref().begin().await?;

    sqlx::query!(
        "DELETE FROM suppliers 
         WHERE id = $1 AND club = $2",
        item_id.id,
        club
    )
    .execute(&mut *pool)
    .await?;

    pool.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
