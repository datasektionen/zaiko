use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

use crate::items::Item;

#[derive(Serialize)]
struct ShortageItem {
    name: String,
    location: String,
    min: f64,
    current_amount: f64,
    order_amount: f64,
}


#[get("/{club}/shortage")]
pub(crate) async fn get_shortage(club: web::Path<String>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
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
pub(crate) async fn take_stock(
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
