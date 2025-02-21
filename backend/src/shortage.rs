use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{auth::check_auth, item::ItemGetResponse};

#[derive(Serialize)]
struct ShortageItem {
    id: i32,
    name: String,
    location: String,
    min: f32,
    current: f32,
    order: f32,
}

#[derive(Deserialize)]
struct StockUpdateRequest {
    items: Vec<(i32, f32)>
}

#[get("/{club}/stock")]
pub(crate) async fn get_shortage(
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    log::info!("get shortage");
    let club = club.as_ref();

    if !check_auth(id, session, club).await {
        return HttpResponse::Unauthorized().finish()
    } 

    let items = match sqlx::query_as!(ItemGetResponse, "SELECT id, name, location, min, max, current, link, supplier, updated FROM items WHERE current <= min AND club = $1", club).fetch_all(pool.get_ref()).await {
        Ok(items) => items,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let items: Vec<ShortageItem> = items
        .iter()
        .filter_map(|item| {
            Some(ShortageItem {
                id: item.id,
                name: item.name.clone(),
                location: item.location.clone(),
                current: item.current,
                order: item.max? - item.current,
                min: item.min?,
            })
        })
        .collect();

    HttpResponse::Ok().json(items)
}

#[post("/{club}/stock")]
pub(crate) async fn take_stock(
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
    body: String,
) -> impl Responder {
    log::info!("update inventory");
    log::debug!("{}", body);

    let club = club.as_ref();

    if !check_auth(id, session, club).await {
        return HttpResponse::Unauthorized().finish()
    } 

    let items: StockUpdateRequest = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    for (id, amount) in items.items {
        if sqlx::query!(
            "UPDATE items SET current = $1 WHERE id = $2 AND club = $3",
            amount,
            id,
            club
        )
        .execute(pool.as_ref())
        .await
        .is_err()
        {
            return HttpResponse::InternalServerError().finish();
        }

        match sqlx::query!(
            "INSERT INTO log (item_id, amount, time, club) VALUES ($1, $2, extract(epoch from now()), $3)",
            id,
            amount,
            club
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(_) => {}
            Err(_) => return HttpResponse::InternalServerError().finish(),
        }
    }

    HttpResponse::Ok().finish()
}
