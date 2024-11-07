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
enum Location {
    Cleaning,
    ToiletCabinet,
    METAdorCabinet,
    Kitchen,
    Unknown,
}

impl From<String> for Location {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Cleaning" => Location::Cleaning,
            "ToiletCabinet" => Location::ToiletCabinet,
            "METAdorCabinet" => Location::METAdorCabinet,
            "Kitchen" => Location::Kitchen,
            _ => Location::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: Option<String>,
    location: Location,
    min: i64,
    max: i64,
    current_amount: i64,
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

#[post("/{club}/item")]
async fn add_item(
    body: String,
    club: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let item: Item = serde_json::from_str(&body).unwrap();
    println!("item: {:?}", item);
    HttpResponse::Ok().body(format!("{:?}", item))
}

#[get("/{club}/items")]
async fn items(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let items = sqlx::query_as!(
        Item,
        "SELECT name, location, min, max, current_amount FROM items"
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    web::Json(items)
}

#[get("/{club}/supplier")]
async fn supplier(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let providors = sqlx::query_as!(
        Supplier,
        "SELECT name, url, order_specification FROM providors"
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    web::Json(providors)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = web::Data::new(SqlitePool::connect("db.sqlite").await.unwrap());
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new().wrap(cors).app_data(pool.clone()).service(
            scope("/api")
                .service(supplier)
                .service(items)
                .service(add_item),
        )
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
