use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
struct Supplier {
    name: String,
    link: Option<String>,
    notes: Option<String>,
    username: Option<String>,
    password: Option<String>,
    club: String,
}

#[get("/{club}/supplier")]
pub(crate) async fn get_supplier(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>, id: web::Query<i64>) -> impl Responder {
    let _club = club.as_ref();
    let id = id.0;

    match sqlx::query_as!(
        Supplier,
        "SELECT name, link, username, password, notes, club FROM suppliers WHERE id = $1",
        id
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/{club}/add_supplier")]
pub(crate) async fn add_supplier(
    body: String,
    club: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let supplier: Supplier = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let club = club.as_ref();

    match sqlx::query!(
        "INSERT INTO suppliers (name, link, notes, username, password, club) VALUES ($1, $2, $3, $4, $5, $6)",
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
        Ok(_) => HttpResponse::Ok().body(format!("{:?}", supplier)),
        Err(_) => HttpResponse::BadRequest().body(format!("{:?}", supplier)),
    }
}

#[post("/{club}/update_supplier")]
pub(crate) async fn update_supplier(club: web::Path<String>, body: String, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let supplier: Supplier = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let club = club.as_ref();

    match sqlx::query!(
        "UPDATE suppliers SET link = $1, notes = $2, username = $3, password = $4 WHERE name = $5 AND club = $6",
        supplier.link,
        supplier.notes,
        supplier.username,
        supplier.password,
        supplier.name,
        club,
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().body(format!("{:?}", supplier)),
        Err(_) => HttpResponse::BadRequest().body(format!("{:?}", supplier)),
    }
}
