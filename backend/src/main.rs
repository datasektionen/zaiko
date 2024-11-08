use std::{borrow::BorrowMut, ops::Deref};

use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, post, scope},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Local},
    Pool, Sqlite, SqlitePool,
};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    location: String,
    min: f64,
    max: f64,
    current_amount: f64,
}

struct Order {
    name: String,
    amount: u32,
    date: DateTime<Local>,
}

struct ItemRef {
    providor: String,
    link: String,
    updated: DateTime<Local>,
}

#[derive(Serialize)]
struct Supplier {
    name: Option<String>,
    url: Option<String>,
    order_specification: Option<String>,
}

#[derive(Serialize)]
struct ShortageItem {
    name: String,
    location: String,
    min: f64,
    current_amount: f64,
    order_amount: f64,
}

#[post("/{club}/item")]
async fn add_item(
    body: String,
    club: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let item: Item = serde_json::from_str(&body).unwrap();
    let club = club.as_ref();
    println!("item: {:?}", item);
    match sqlx::query!(
        "INSERT INTO items (name, location, min, max, current_amount, club) VALUES ($1, $2, $3, $4, $5, $6)",
        item.name,
        item.location,
        item.min,
        item.max,
        item.current_amount,
        club
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().body(format!("{:?}", item)),
        Err(_) => HttpResponse::BadRequest().body(format!("{:?}", item)),
    }
}

#[get("/{club}/items")]
async fn get_items(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let club = club.as_ref();
    let items = sqlx::query_as!(
        Item,
        "SELECT name, location, min, max, current_amount FROM items WHERE club = $1",
        club
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    web::Json(items)
}

#[get("/{club}/shortage")]
async fn get_shortage(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let club = club.as_ref();
    let items = sqlx::query_as!(
        Item,
        "SELECT name, location, min, max, current_amount FROM items WHERE current_amount <= min AND club = $1",
        club
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    let items: Vec<ShortageItem> = items
        .iter()
        .map(|item| ShortageItem {
            name: item.name.clone(),
            location: item.location.clone(),
            current_amount: item.current_amount,
            order_amount: item.max - item.current_amount,
            min: item.min,
        })
        .collect();

    web::Json(items)
}

// #[get("/{club}/supplier")]
// async fn get_suppliers(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
//     let providors = sqlx::query_as!(
//         Supplier,
//         "SELECT name, url, order_specification FROM providors"
//     )
//     .fetch_all(pool.get_ref())
//     .await
//     .unwrap();
//
//     web::Json(providors)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = web::Data::new(SqlitePool::connect("db.sqlite").await.unwrap());
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new().wrap(cors).app_data(pool.clone()).service(
            scope("/api")
                // .service(get_suppliers)
                .service(get_items)
                .service(get_shortage)
                .service(add_item),
        )
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
