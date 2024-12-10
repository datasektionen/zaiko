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

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use sqlx::SqlitePool;

    use super::{add_item, get_items, update_item, AddItem};

    #[actix_web::test]
    async fn test_get_all_items() {
        let pool = web::Data::new(
            SqlitePool::connect("db.sqlite")
                .await
                .expect("Expected sqlite database with name db.sqlite"),
        );

        let app = test::init_service(App::new().app_data(pool).service(get_items)).await;
        let req = test::TestRequest::get().uri("/metadorerna/items").to_request();
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

        let body = AddItem {
            name: String::from("tejp"),
            min: Some(10.0),
            max: Some(20.0),
            current: 15.0,
            location: String::from("Unknown"),
            supplier: None,
            link: None,
        };

        let app = test::init_service(App::new().app_data(pool).service(add_item)).await;
        let req = test::TestRequest::post().uri("/metadorerna/item").set_json(body).to_request();
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

        let mut body = AddItem {
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
        let req = test::TestRequest::post().uri("/metadorerna/update").set_json(body).to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }
}
