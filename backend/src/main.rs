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
