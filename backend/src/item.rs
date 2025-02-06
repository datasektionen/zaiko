use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ItemGetResponse {
    pub(crate) id: i64,
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
pub(crate) struct ItemAddRequest {
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) min: Option<f64>,
    pub(crate) max: Option<f64>,
    pub(crate) current: f64,
    pub(crate) supplier: Option<i64>,
    pub(crate) link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ItemUpdateRequest {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) min: Option<f64>,
    pub(crate) max: Option<f64>,
    pub(crate) current: f64,
    pub(crate) supplier: Option<i64>,
    pub(crate) link: Option<String>,
}

#[get("/{club}/item")]
pub(crate) async fn get_item(
    club: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    let club = club.as_ref();
    match sqlx::query_as!(
        ItemGetResponse,
        "SELECT id, name, location, min, max, current, link, supplier, updated FROM items WHERE club = $1",
        club
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/{club}/item")]
pub(crate) async fn add_item(
    body: String,
    club: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let item: ItemAddRequest = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let club = club.as_ref();

    let res = match sqlx::query!(
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    };

    let id = match sqlx::query!(
        "SELECT id FROM items WHERE name = $1 AND club = $2",
        item.name,
        club
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(id) => id.id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match sqlx::query!(
        "INSERT INTO log (id, amount, time, club) VALUES ($1, $2, strftime('%s', 'now'), $3)",
        id,
        item.current,
        club
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {}
        Err(_) => return HttpResponse::BadRequest().finish(),
    }

    res
}

#[patch("/{club}/item")]
pub(crate) async fn update_item(
    club: web::Path<String>,
    body: String,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    let item: ItemUpdateRequest = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let club = club.as_ref();

    let current = match sqlx::query!(
        "SELECT current FROM items WHERE name = $1 AND club = $2",
        item.name,
        club
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(current) => current.current,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if current != item.current {
        match sqlx::query!(
            "INSERT INTO log (id, amount, time, club) VALUES ($1, $2, strftime('%s', 'now'), $3)",
            item.id,
            item.current,
            club
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(_) => {}
            Err(_) => return HttpResponse::BadRequest().finish(),
        }
    }

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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[delete("/{club}/item")]
pub(crate) async fn delete_item(
    club: web::Path<String>,
    id: web::Query<i64>,
    pool: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    let club = club.as_ref();
    match sqlx::query!("DELETE FROM items WHERE id = $1 AND club = $2", id.0, club)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => {}
        Err(_) => return HttpResponse::BadRequest().finish(),
    }

    match sqlx::query!("DELETE FROM log WHERE id = $1 AND club = $2", id.0, club)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use sqlx::SqlitePool;

    use super::{add_item, get_item, update_item, ItemAddRequest};

    #[actix_web::test]
    async fn test_get_all_items() {
        let pool = web::Data::new(
            SqlitePool::connect("db.sqlite")
                .await
                .expect("Expected sqlite database with name db.sqlite"),
        );

        let app = test::init_service(App::new().app_data(pool).service(get_item)).await;
        let req = test::TestRequest::get()
            .uri("/metadorerna/items")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn test_add_item() {
        let pool = web::Data::new(
            SqlitePool::connect("db.sqlite")
                .await
                .expect("Expected sqlite database with name db.sqlite"),
        );

        let body = ItemAddRequest {
            name: String::from("tejp"),
            min: Some(10.0),
            max: Some(20.0),
            current: 15.0,
            location: String::from("Unknown"),
            supplier: None,
            link: None,
        };

        let app = test::init_service(App::new().app_data(pool).service(add_item)).await;
        let req = test::TestRequest::post()
            .uri("/metadorerna/item")
            .set_json(body)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn test_update_item() {
        let pool = web::Data::new(
            SqlitePool::connect("db.sqlite")
                .await
                .expect("Expected sqlite database with name db.sqlite"),
        );

        let mut body = ItemAddRequest {
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
            "UPDATE items SET location = $1, min = $2, max = $3, current = $4, supplier = $5, updated = strftime('%s', 'now'), link = $6  WHERE name = $7 AND club = $8",
            body.location,
            body.min,
            body.max,
            body.current,
            body.supplier,
            body.link,
            body.name,
            club,
        )
        .execute(pool.get_ref())
        .await.unwrap();

        body.location = String::from("Metador closet");

        let app = test::init_service(App::new().app_data(pool).service(update_item)).await;
        let req = test::TestRequest::post()
            .uri("/metadorerna/update")
            .set_json(body)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }
}
