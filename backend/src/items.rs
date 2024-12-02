use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Sqlite};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Item {
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) min: Option<f64>,
    pub(crate) max: Option<f64>,
    pub(crate) current: f64,
    pub(crate) supplier: Option<i64>,
    pub(crate) updated: i64,
    pub(crate) link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AddItem {
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) min: Option<f64>,
    pub(crate) max: Option<f64>,
    pub(crate) current: f64,
    pub(crate) supplier: Option<String>,
    pub(crate) link: Option<String>,
}

#[post("/{club}/item")]
pub(crate) async fn add_item(
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
pub(crate) async fn update_item(club: web::Path<String>, body: String, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
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
pub(crate) async fn get_items(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
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
