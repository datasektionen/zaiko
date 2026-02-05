use actix_web::{
    delete, get, patch, post, put,
    web::{self},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::{IntoParams, ToSchema};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    auth::{check_auth, get_permitted_storages, types::HivePermission, CheckType},
    db::{
        self,
        interval::Interval,
        item::{BasicItem, DetailedItem},
    },
    error::Error,
};

/// Info to add an item to storage
#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct ItemAddRequest {
    /// The storage location the item is at
    storage: String,
    /// The container the item is stored in, can be "default" (loose)
    container: String,
    /// The items name
    name: String,
    /// The lower limit on how many items should be in storage (order limit)
    min: Option<f32>,
    /// The upper limit on the number of items (order ceiling)
    max: Option<f32>,
    /// The number of items currently in storage
    amount: f32,
    /// The unit in which amount is counted (st, 6-pack, %, etc)
    unit: Option<String>,
    /// The time between the item should be inventoried
    inventory_interval: Option<Interval>,
}

/// Info used to add a supplier to an item
#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct SupplierAddRequest {
    /// The suppliers name
    supplier: String,
    /// The items name
    name: String,
    /// Link to the item och the suppliers webshop
    link: Option<String>,
    /// If it is prefered to buy from this supplier instead of the others
    prefered: bool,
}

/// Info to change the name of an item (accross all storages)
#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct ItemChangeNameRequest {
    /// The name to change from
    name: String,
    /// The name to change to
    new_name: Option<String>,
    /// The unit amount is counted in
    unit: String,
    /// The interval between the item needs to be inventoried
    inventory_interval: Option<Interval>,
}

/// Info used to move an item
#[derive(Debug, Deserialize, ToSchema)]
struct ItemMoveRequest {
    /// The items name
    name: String,
    /// The amount of items to move
    amount: f32,
    /// The name of the storage the items is moved from
    from_storage: String,
    /// The name of the container the items is moved from
    from_container: String,
    /// The name of the storage the items is moved to
    to_storage: String,
    /// The name of the container the items is moved to
    to_container: String,
}

/// Info used to update an item at a particular storage location
#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct StoredUpdateRequest {
    /// The storage the item is located in
    storage: String,
    /// The storage to move the item to
    new_storage: Option<String>,
    /// The container the item is located in
    container: String,
    /// The container to move the item to
    new_container: Option<String>,
    /// The items name
    name: String,
    /// The lower limit on how many items should be in storage (order limit)
    min: Option<f32>,
    /// The upper limit on the number of items (order ceiling)
    max: Option<f32>,
    /// The current amount in storage
    amount: f32,
}

/// Info used when filtering the items list
#[derive(Deserialize, Debug, IntoParams, ToSchema)]
struct ItemsGetQuery {
    /// The items name
    name: Option<String>,
    /// The storages name
    storage: Option<String>,
    /// The containers name (requier storage to also be set)
    container: Option<String>,
    /// The suppliers name
    supplier: Option<String>,
    /// Atleast this many items should exist
    min: Option<f32>,
    /// Atmost this many items should exist
    max: Option<f32>,
}

/// Info used to get a specific item
#[derive(Deserialize, Debug, IntoParams)]
struct ItemGetQuery {
    /// The items name
    name: String,
}

/// Info used to delete an item from storage
#[derive(Debug, Deserialize, IntoParams)]
struct ItemDeleteQuery {
    /// The items name
    name: String,
    /// The storage locations name
    storage: String,
    /// The containers name
    container: String,
}

/// Info used to remove a supplier from an item
#[derive(Debug, Deserialize, IntoParams)]
struct SupplierRemoveQuery {
    /// The items name
    name: String,
    /// The suppliers name
    supplier: String,
}

pub fn config() -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut ServiceConfig| {
        cfg.service(get_items)
            .service(get_item)
            .service(create_item)
            .service(change_item)
            .service(change_stored_item)
            .service(move_item)
            .service(supply_item)
            .service(change_supply_item)
            .service(delete_item)
            .service(unsupply_item);
    }
}

#[utoipa::path(
    tag = "item",
    params(ItemsGetQuery),
    responses(
        (
            status = StatusCode::OK,
            body = Vec<BasicItem>,
            description = "List of items with basic info"
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
#[get("/items")]
async fn get_items(
    db: web::Data<Pool<Postgres>>,
    query: web::Query<ItemsGetQuery>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let permitted_storages = get_permitted_storages(&db, &permissions).await?;

    let items = db::item::get_all_filtered_basic(
        &db,
        query.name.as_deref(),
        query.storage.as_deref(),
        query.container.as_deref(),
        query.supplier.as_deref(),
        query.min,
        query.max,
        &permitted_storages,
    )
    .await?;
    Ok(HttpResponse::Ok().json(items))
}

#[utoipa::path(
    tag = "item",
    params(ItemGetQuery),
    responses(
        (
            status = StatusCode::OK,
            body = DetailedItem,
            description = "Item with detailed info"
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
#[get("/item")]
async fn get_item(
    db: web::Data<Pool<Postgres>>,
    query: web::Query<ItemGetQuery>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let permitted_storages = get_permitted_storages(&db, &permissions).await?;

    let item = db::item::get_item_by_name_detailed(&db, &query.name, &permitted_storages).await?;
    Ok(HttpResponse::Ok().json(item))
}

#[utoipa::path(
    tag = "item",
    request_body = ItemAddRequest,
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
            status = StatusCode::UNAUTHORIZED,
            description = "Unauthorized"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[post("/item")]
async fn create_item(
    body: String,
    db: web::Data<Pool<Postgres>>,
    id: web::ReqData<String>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let item: ItemAddRequest = serde_json::from_str(&body)?;

    check_auth(
        CheckType::Storage {
            storage: &item.storage,
            container: Some(&item.container),
        },
        &db,
        &permissions,
    )
    .await?;

    db::item::create(
        &db,
        &id,
        &item.storage,
        &item.container,
        &item.name,
        item.min,
        item.max,
        item.amount,
        item.unit.as_deref(),
        item.inventory_interval,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "item",
    request_body = SupplierAddRequest,
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
            status = StatusCode::UNAUTHORIZED,
            description = "Unauthorized"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[post("/supply")]
async fn supply_item(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let supplier: SupplierAddRequest = serde_json::from_str(&body)?;

    check_auth(CheckType::Item(&supplier.name), &db, &permissions).await?;

    db::item::add_supplier(
        &db,
        &supplier.supplier,
        &supplier.name,
        supplier.link.as_deref(),
        supplier.prefered,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "item",
    request_body = SupplierAddRequest,
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
            status = StatusCode::UNAUTHORIZED,
            description = "Unauthorized"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[patch("/supply")]
async fn change_supply_item(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let supplier: SupplierAddRequest = serde_json::from_str(&body)?;

    check_auth(CheckType::Item(&supplier.name), &db, &permissions).await?;

    db::item::update_supplier(
        &db,
        &supplier.supplier,
        &supplier.name,
        supplier.link.as_deref(),
        supplier.prefered,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "item",
    request_body = ItemChangeNameRequest,
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
            status = StatusCode::UNAUTHORIZED,
            description = "Unauthorized"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[patch("/item")]
async fn change_item(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let item: ItemChangeNameRequest = serde_json::from_str(&body)?;
    check_auth(CheckType::Item(&item.name), &db, &permissions).await?;

    db::item::change(
        &db,
        &item.name,
        item.new_name.as_deref(),
        &item.unit,
        item.inventory_interval,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "item",
    request_body = StoredUpdateRequest,
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
            status = StatusCode::UNAUTHORIZED,
            description = "Unauthorized"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[put("/item")]
async fn change_stored_item(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
    id: web::ReqData<String>,
) -> Result<HttpResponse, Error> {
    let stored_item: StoredUpdateRequest = serde_json::from_str(&body)?;
    check_auth(CheckType::Item(&stored_item.name), &db, &permissions).await?;

    db::item::change_stored_item(
        &db,
        &stored_item.name,
        stored_item.amount,
        stored_item.min,
        stored_item.max,
        &stored_item.storage,
        stored_item.new_storage.as_deref(),
        &stored_item.container,
        stored_item.new_container.as_deref(),
        &id,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "item",
    request_body = ItemMoveRequest,
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
            status = StatusCode::UNAUTHORIZED,
            description = "Unauthorized"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[patch("/item/move")]
async fn move_item(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
    id: web::ReqData<String>,
) -> Result<HttpResponse, Error> {
    let item: ItemMoveRequest = serde_json::from_str(&body)?;

    check_auth(
        CheckType::MoveItem {
            from_storage: &item.from_storage,
            from_container: &item.from_container,
            to_storage: &item.to_storage,
            to_container: &item.to_container,
        },
        &db,
        &permissions,
    )
    .await?;

    let mut db = db.begin().await?;

    db::item::move_item(
        &mut db,
        &item.name,
        Some(item.amount),
        &item.from_storage,
        &item.from_container,
        &item.to_storage,
        &item.to_container,
        &id,
    )
    .await?;

    db.commit().await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "item",
    params(ItemDeleteQuery),
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
            status = StatusCode::UNAUTHORIZED,
            description = "Unauthorized"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[delete("/item")]
async fn delete_item(
    query: web::Query<ItemDeleteQuery>,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    check_auth(
        CheckType::Storage {
            storage: &query.storage,
            container: Some(&query.container),
        },
        &db,
        &permissions,
    )
    .await?;
    db::item::delete(&db, &query.storage, &query.container, &query.name).await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "item",
    params(SupplierRemoveQuery),
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
            status = StatusCode::UNAUTHORIZED,
            description = "Unauthorized"
        ),
        (
            status = StatusCode::INTERNAL_SERVER_ERROR,
            description = "Internal Server Error"
        )
    )
)]
#[delete("/supply")]
async fn unsupply_item(
    query: web::Query<SupplierRemoveQuery>,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    check_auth(CheckType::Item(&query.name), &db, &permissions).await?;
    db::item::delete_supplier(&db, &query.name, &query.supplier).await?;
    Ok(HttpResponse::Ok().finish())
}
