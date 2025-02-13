use actix_identity::Identity;
use actix_session::Session;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::auth::check_auth;

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
    id: Option<i64>
}

#[get("/{club}/supplier")]
pub(crate) async fn get_supplier(
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
    query: web::Query<Query>,
) -> impl Responder {
    log::info!("get supplier");

    let club = club.as_ref();

    if !check_auth(id, session, club).await {
        return HttpResponse::Unauthorized().finish()
    } 

    if let Some(id) = query.id {
        match sqlx::query!("SELECT name FROM suppliers WHERE id = $1", id)
            .fetch_one(pool.get_ref())
            .await
        {
            Ok(items) => HttpResponse::Ok().json(items.name),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    } else {
        match sqlx::query_as!(
            SupplierGetResponse,
            "SELECT id, name, username, password, link, notes, updated FROM suppliers WHERE club = $1",
            club
        )
        .fetch_all(pool.get_ref())
        .await
        {
            Ok(supplier) => HttpResponse::Ok().json(supplier),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[get("/{club}/suppliers")]
pub(crate) async fn get_suppliers(club: web::Path<String>, id: Option<Identity>, session: Session, pool: web::Data<Pool<Sqlite>>) -> HttpResponse {
    log::info!("get suppliers");

    let club = club.as_ref();
    
    if !check_auth(id, session, club).await {
        return HttpResponse::Unauthorized().finish()
    } 

    match sqlx::query_as!(
        SupplierListGetResponse,
        "SELECT id, name FROM suppliers WHERE club = $1",
        club
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(supplier) => HttpResponse::Ok().json(supplier),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/{club}/supplier")]
pub(crate) async fn add_supplier(
    body: String,
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    log::info!("add supplier");
    log::debug!("{}", body);

    let club = club.as_ref();

    if !check_auth(id, session, club).await {
        return HttpResponse::Unauthorized().finish()
    } 

    let supplier: SupplierAddRequest = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match sqlx::query!(
        "INSERT INTO suppliers (name, link, notes, username, password, updated, club) VALUES ($1, $2, $3, $4, $5, strftime('%s', 'now'), $6)",
        supplier.name,
        supplier.link,
        supplier.notes,
        supplier.username,
        supplier.password,
        club,
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/{club}/supplier")]
pub(crate) async fn update_supplier(
    club: web::Path<String>,
    body: String,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    log::info!("update supplier");
    log::debug!("{}", body);

    let club = club.as_ref();

    if !check_auth(id, session, club).await {
        return HttpResponse::Unauthorized().finish()
    } 

    let supplier: SupplierUpdateRequest = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match sqlx::query!(
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
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{club}/supplier")]
pub(crate) async fn delete_supplier(
    club: web::Path<String>,
    item_id: web::Query<i64>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    let club = club.as_ref();

    if !check_auth(id, session, club).await {
        return HttpResponse::Unauthorized().finish()
    } 

    match sqlx::query!(
        "DELETE FROM suppliers WHERE id = $1 AND club = $2",
        item_id.0,
        club
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
