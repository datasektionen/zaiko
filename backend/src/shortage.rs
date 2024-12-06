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
    let items: Vec<(String, f64)> = match serde_json::from_str(&body) {
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

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use sqlx::SqlitePool;

    use crate::items::AddItem;

    use super::{get_shortage, take_stock};

    #[actix_web::test]
    async fn test_shortage() {
        let pool = web::Data::new(
            SqlitePool::connect("db.sqlite")
                .await
                .expect("Expected sqlite database with name db.sqlite"),
        );

        let item = AddItem {
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
        let req = test::TestRequest::get().uri("/metadorerna/shortage").to_request();
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

        let item = AddItem {
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

        let body = vec![(String::from("tejp"), 5.0)];

        let app = test::init_service(App::new().app_data(pool).service(take_stock)).await;
        let req = test::TestRequest::post().uri("/metadorerna/take_stock").set_json(body).to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }
}
