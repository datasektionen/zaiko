use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, scope},
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
    min: Option<f64>,
    max: Option<f64>,
    current: f64,
    supplier: Option<i64>,
    updated: i64,
    link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddItem {
    name: String,
    location: String,
    min: Option<f64>,
    max: Option<f64>,
    current: f64,
    supplier: Option<String>,
    link: Option<String>,
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
    let item: AddItem = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let club = club.as_ref();

    match sqlx::query!(
        "INSERT INTO items (name, location, min, max, current, supplier, updated, link, club) VALUES ($1, $2, $3, $4, $5, $6, strftime('%s', 'now'), $7, $8)",
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
    .await
    {
        Ok(_) => HttpResponse::Ok().body(format!("{:?}", item)),
        Err(_) => HttpResponse::BadRequest().body(format!("{:?}", item)),
    }
}

#[post("/{club}/update")]
async fn update_item(club: web::Path<String>, body: String, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let item: AddItem = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let club = club.as_ref();

    match sqlx::query!(
        "UPDATE items SET location = $1, min = $2, max = $3, current = $4, supplier = $5, updated = strftime('%s', 'now'), link = $6  WHERE name = $7 AND club = $8",
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
    .await
    {
        Ok(_) => HttpResponse::Ok().body(format!("{:?}", item)),
        Err(_) => HttpResponse::BadRequest().body(format!("{:?}", item)),
    }
}

#[get("/{club}/items")]
async fn get_items(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let club = club.as_ref();
    match sqlx::query_as!(
        Item,
        "SELECT name, location, min, max, current, link, supplier, updated FROM items WHERE club = $1",
        club
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{club}/shortage")]
async fn get_shortage(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let club = club.as_ref();
    let items = match sqlx::query_as!(Item,"SELECT name, location, min, max, current, link, supplier, updated FROM items WHERE current <= min AND club = $1",club).fetch_all(pool.get_ref()).await {
        Ok(items) => items,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let items: Vec<ShortageItem> = items
        .iter()
        .filter_map(|item| {
            Some(ShortageItem {
                name: item.name.clone(),
                location: item.location.clone(),
                current_amount: item.current,
                order_amount: item.max? - item.current,
                min: item.min?,
            })
        })
        .collect();

    HttpResponse::Ok().json(items)
}

#[post("/{club}/take_stock")]
async fn take_stock(
    club: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
    body: String,
) -> impl Responder {
    let items: Vec<(String, i64)> = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let club = club.as_ref();

    for item in items {
        if sqlx::query!(
            "UPDATE items SET current = $1 WHERE name = $2 AND club = $3",
            item.1,
            item.0,
            club
        )
        .execute(pool.as_ref())
        .await
        .is_err()
        {
            return HttpResponse::InternalServerError().finish();
        }
    }

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = web::Data::new(
        SqlitePool::connect("db.sqlite")
            .await
            .expect("Expected sqlite database with name db.sqlite"),
    );
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new().wrap(cors).app_data(pool.clone()).service(
            scope("/api")
                // .service(get_suppliers)
                .service(get_items)
                .service(get_shortage)
                .service(add_item)
                .service(take_stock)
                .service(update_item),
        )
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
