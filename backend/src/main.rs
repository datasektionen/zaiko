use actix_cors::Cors;
use actix_web::{
    web::{self, scope},
    App, HttpServer,
};
use sqlx::SqlitePool;

mod item;
mod log;
mod serve;
mod shortage;
mod supplier;

use crate::item::{add_item, delete_item, get_item, update_item};
use crate::log::get_log;
use crate::serve::serve_frontend;
use crate::shortage::{get_shortage, take_stock};
use crate::supplier::{add_supplier, delete_supplier, get_supplier, update_supplier};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let pool = web::Data::new(
        SqlitePool::connect("db.sqlite")
            .await
            .expect("Expected sqlite database with name db.sqlite"),
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .app_data(pool.clone())
            .service(
                scope("/api")
                    .service(get_item)
                    .service(add_item)
                    .service(update_item)
                    .service(delete_item)
                    .service(get_supplier)
                    .service(add_supplier)
                    .service(update_supplier)
                    .service(delete_supplier)
                    .service(get_shortage)
                    .service(take_stock)
                    .service(get_log),
            )
            .service(serve_frontend)
            .service(actix_files::Files::new("/", "../dist/").index_file("index.html"))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
