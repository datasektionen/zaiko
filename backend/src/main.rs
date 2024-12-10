use actix_cors::Cors;
use actix_web::{
    web::{self, scope},
    App, HttpServer,
};
use sqlx::SqlitePool;

mod items;
mod supplier;
mod shortage;

use crate::items::{get_items, update_item, add_item};
use crate::shortage::{get_shortage, take_stock};
use crate::supplier::{get_supplier, add_supplier, update_supplier};

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
                .service(get_items)
                .service(add_item)
                .service(update_item)
                .service(get_shortage)
                .service(take_stock)
                .service(get_supplier)
                .service(add_supplier)
                .service(update_supplier)
        )
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
