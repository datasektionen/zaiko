use actix_identity::Identity;
use actix_session::Session;
use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::{auth::check_auth, error::Error};

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
    id: Option<Identity>,
    session: Session,
) -> Result<HttpResponse, Error> {
    log::info!("get items");
    let club = club.as_ref();

    check_auth(id, session, club).await?;

    let items = sqlx::query_as!(
        ItemGetResponse,
        "SELECT id, name, location, min, max, current, link, supplier, updated FROM items WHERE club = $1",
        club
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("/{club}/item")]
pub(crate) async fn add_item(
    body: String,
    club: web::Path<String>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, Error> {
    log::info!("add item");
    log::debug!("{}", body);

    let club = club.as_ref();

    check_auth(id, session, club).await?;

    let item: ItemAddRequest = serde_json::from_str(&body)?;

    if item.name.is_empty() && item.location.is_empty() {
        return Err(Error::BadRequest);
    }

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
    .await?;

    let id = sqlx::query!(
        "SELECT id FROM items WHERE name = $1 AND club = $2",
        item.name,
        club
    )
    .fetch_one(pool.get_ref())
    .await?
    .id;

    sqlx::query!(
        "INSERT INTO log (item_id, amount, time, club) VALUES ($1, $2, strftime('%s', 'now'), $3)",
        id,
        item.current,
        club
    )
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[patch("/{club}/item")]
pub(crate) async fn update_item(
    club: web::Path<String>,
    body: String,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, Error> {
    log::info!("update item");
    log::debug!("{}", body);

    let club = club.as_ref();

    check_auth(id, session, club).await?;

    let item: ItemUpdateRequest = serde_json::from_str(&body)?;

    if item.name.is_empty() && item.location.is_empty() {
        return Err(Error::BadRequest);
    }

    let current = sqlx::query!(
        "SELECT current FROM items WHERE name = $1 AND club = $2",
        item.name,
        club
    )
    .fetch_one(pool.as_ref())
    .await?
    .current;

    if current != item.current {
        sqlx::query!(
            "INSERT INTO log (item_id, amount, time, club) VALUES ($1, $2, strftime('%s', 'now'), $3)",
            item.id,
            item.current,
            club
        )
        .execute(pool.get_ref())
        .await?;
    }

    sqlx::query!(
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
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/{club}/item")]
pub(crate) async fn delete_item(
    club: web::Path<String>,
    item_id: web::Query<i64>,
    id: Option<Identity>,
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, Error> {
    let club = club.as_ref();

    check_auth(id, session, club).await?;

    sqlx::query!(
        "DELETE FROM items WHERE id = $1 AND club = $2",
        item_id.0,
        club
    )
    .execute(pool.get_ref())
    .await?;

    sqlx::query!(
        "DELETE FROM log WHERE item_id = $1 AND club = $2",
        item_id.0,
        club
    )
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
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
