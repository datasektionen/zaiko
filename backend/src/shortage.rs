use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use utoipa::{IntoParams, ToSchema};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    auth::{get_permitted_storages, types::HivePermission},
    db::{
        self,
        item::{DueStorage, ShortageItem},
    },
    error::Error,
};

/// Info used to take inventory
#[derive(Deserialize, ToSchema)]
struct StockUpdateRequest {
    /// List of items on which to update the amount
    items: Vec<StockUpdate>,
}

#[derive(Deserialize, ToSchema)]
struct StockUpdate {
    /// The items name
    name: String,
    /// The storage where the item is located
    storage: String,
    /// The container where the item is stored
    container: String,
    /// The number of items currently in storage
    amount: f32,
}

pub fn config() -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut ServiceConfig| {
        cfg.service(get_shortage)
            .service(items_due)
            .service(take_stock);
    }
}

#[utoipa::path(
    tag = "inventory",
    responses(
        (
            status = StatusCode::OK,
            body = Vec<ShortageItem>,
            description = "List of items with less than order floor"
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Bad Request"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[get("/shortage")]
async fn get_shortage(
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let protected: Vec<String> = permissions
        .iter()
        .filter_map(|perm| {
            if perm.id == "read" {
                perm.scope.clone()
            } else {
                None
            }
        })
        .collect();

    let items = db::item::get_shortage(&db, &protected).await?;

    Ok(HttpResponse::Ok().json(items))
}

#[utoipa::path(
    tag = "inventory",
    responses(
        (
            status = StatusCode::OK,
            body = Vec<DueStorage>,
            description = "List of items due to be inventoried"
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Bad Request"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[get("/inventory")]
async fn items_due(
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let permitted_storages = get_permitted_storages(&db, &permissions).await?;

    let items = db::item::items_due(&db, &permitted_storages).await?;

    Ok(HttpResponse::Ok().json(items))
}

#[utoipa::path(
    tag = "inventory",
    request_body = StockUpdateRequest,
    responses(
        (
            status = StatusCode::OK,
            description = "Success"
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Bad Request"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[post("/inventory")]
async fn take_stock(
    db: web::Data<Pool<Postgres>>,
    id: web::ReqData<String>,
    body: String,
) -> Result<HttpResponse, Error> {
    let mut db = db.get_ref().begin().await?;

    let items: StockUpdateRequest = serde_json::from_str(&body)?;

    for StockUpdate {
        name,
        storage,
        container,
        amount,
    } in items.items
    {
        db::item::update_amount_in_transaction(&mut db, &id, &name, &storage, &container, amount)
            .await?;
    }

    db.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
