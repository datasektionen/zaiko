use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::item::ItemGetResponse;

#[derive(Serialize)]
struct ShortageItem {
    id: i64,
    name: String,
    location: String,
    min: f64,
    current: f64,
    order: f64,
}

#[derive(Deserialize)]
struct StockUpdateRequest {
    items: Vec<(i64, f64)>
}

#[get("/{club}/stock")]
pub(crate) async fn get_shortage(
    club: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    log::info!("get shortage");
    let club = club.as_ref();
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
    pool: web::Data<Pool<Sqlite>>,
    body: String,
) -> impl Responder {
    log::info!("update inventory");
    log::debug!("{}", body);
    let items: StockUpdateRequest = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let club = club.as_ref();

    for (id, amount) in items.items {
        if sqlx::query!(
            "UPDATE items SET current = $1 WHERE id = $2 AND club = $3",
            id,
            amount,
            club
        )
        .execute(pool.as_ref())
        .await
        .is_err()
        {
            return HttpResponse::InternalServerError().finish();
        }

        match sqlx::query!(
            "INSERT INTO log (id, amount, time, club) VALUES ($1, $2, strftime('%s', 'now'), $3)",
            id,
            amount,
            club
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(_) => {}
            Err(_) => return HttpResponse::BadRequest().finish(),
        }
    }

    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use sqlx::SqlitePool;

    use crate::item::ItemAddRequest;

    use super::{get_shortage, take_stock};

    #[actix_web::test]
    async fn test_shortage() {
        let pool = web::Data::new(
            SqlitePool::connect("db.sqlite")
                .await
                .expect("Expected sqlite database with name db.sqlite"),
        );

        let item = ItemAddRequest {
            name: String::from("tejp"),
            min: Some(10.0),
            max: Some(20.0),
            current: 5.0,
            location: String::from("Unknown"),
            supplier: None,
            link: None,
        };

        let club = String::from("metadorerna");

        sqlx::query!(
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
        .await.unwrap();

        let app = test::init_service(App::new().app_data(pool).service(get_shortage)).await;
        let req = test::TestRequest::get()
            .uri("/metadorerna/shortage")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn test_take_stock() {
        let pool = web::Data::new(
            SqlitePool::connect("db.sqlite")
                .await
                .expect("Expected sqlite database with name db.sqlite"),
        );

        let item = ItemAddRequest {
            name: String::from("tejp"),
            min: Some(10.0),
            max: Some(20.0),
            current: 15.0,
            location: String::from("Unknown"),
            supplier: None,
            link: None,
        };

        let club = String::from("metadorerna");

        sqlx::query!(
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
        .await.unwrap();

        let mut body = Vec::new();
        body.push((String::from("tejp"), 15.0));

        let app = test::init_service(App::new().app_data(pool).service(take_stock)).await;
        let req = test::TestRequest::post()
            .uri("/metadorerna/take_stock")
            .set_json(body)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }
}
