use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use utoipa::{IntoParams, ToSchema};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    auth::{check_auth, types::HivePermission, CheckType},
    db::{
        self,
        container::{ContainerItem, ContainerStorage},
        interval::Interval,
        storage::Storage,
    },
    error::Error,
};

/// Used to get a specific storage
#[derive(Debug, Deserialize, IntoParams)]
struct StorageGetQuery {
    /// The name of the requested storage
    name: String,
}

/// Info used to create a storage location
#[derive(Deserialize, ToSchema)]
struct StorageCreateRequest {
    /// The suppliers name
    name: String,
    /// If the storage should require a read permission
    protected: bool,
    /// The time between the storage should be inventoried
    inventory_interval: Option<Interval>,
}

/// Info used to update a storage location
#[derive(Deserialize, ToSchema)]
struct StorageUpdateRequest {
    /// The suppliers name
    name: String,
    /// The new name of the storage if it is changed
    new_name: Option<String>,
    /// If the storage should require a read permission
    protected: bool,
    /// The time between when the storage should be inventoried
    inventory_interval: Option<Interval>,
}

/// Info used to delete a storage location
#[derive(Deserialize, IntoParams)]
struct StorageDeleteQuery {
    /// The storages name
    name: String,
}

/// Info used to create a container
#[derive(Deserialize, ToSchema)]
struct ContainerCreateRequest {
    /// The containers name
    name: String,
    /// The storage location the container is at
    storage: String,
    /// The time between when the container should be inventoried
    inventory_interval: Option<Interval>,
}

/// Info used to update a container
#[derive(Deserialize, ToSchema)]
struct ContainerUpdateRequest {
    /// The containers name
    name: String,
    /// The containers new name if it should be changed
    new_name: Option<String>,
    /// The storage location the container should be at
    storage: String,
}

/// Info used to move a container
#[derive(Deserialize, ToSchema)]
struct ContainerMoveRequest {
    /// The containers name
    name: String,
    /// The name of the storage the container is moved from
    from_storage: String,
    /// The name of the storage the container is moved to
    to_storage: String,
    /// Spcifies if containers with the same name should be merged
    merge: bool,
}

/// Info used to delete a container
#[derive(Deserialize, IntoParams)]
struct ContainerDeleteQuery {
    /// The containers name
    name: String,
    /// The name of the storage the container is at
    storage: String,
}

pub(crate) fn config() -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut ServiceConfig| {
        cfg.service(get_storages)
            .service(get_storage_container_tree)
            .service(get_container_item_tree_for_storage)
            .service(create_storage)
            .service(change_storage)
            .service(destroy_storage)
            .service(create_container)
            .service(change_container)
            .service(move_container)
            .service(destroy_container);
    }
}

#[utoipa::path(
    tag = "storage",
    responses(
        (
            status = StatusCode::OK,
            body = Vec<Storage>,
            description = "List of storages with information"
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
#[get("/storages")]
async fn get_storages(
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let storages = if check_auth(CheckType::Admin, &db, &permissions)
        .await
        .is_ok()
    {
        db::storage::get_all_unprotected(&db).await?
    } else {
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

        db::storage::get_all(&db, &protected).await?
    };

    Ok(HttpResponse::Ok().json(storages))
}

#[utoipa::path(
    tag = "storage",
    responses(
        (
            status = StatusCode::OK,
            body = Vec<ContainerStorage>,
            description = "List of storages with their containers"
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
#[get("/storages/containers")]
async fn get_storage_container_tree(
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

    let storages: Vec<ContainerStorage> =
        db::container::get_all_containers_grouped_by_storage(&db, &protected).await?;

    Ok(HttpResponse::Ok().json(storages))
}

#[utoipa::path(
    tag = "storage",
    params(StorageGetQuery),
    responses(
        (
            status = StatusCode::OK,
            body = Vec<ContainerItem>,
            description = "List of storages with their containers"
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
#[get("/storages/containers/items")]
async fn get_container_item_tree_for_storage(
    db: web::Data<Pool<Postgres>>,
    query: web::Query<StorageGetQuery>,
) -> Result<HttpResponse, Error> {
    let storage = db::container::get_all_containers_in_storage_with_items(&db, &query.name).await?;

    Ok(HttpResponse::Ok().json(storage))
}

#[utoipa::path(
    tag = "storage",
    request_body = StorageCreateRequest,
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
#[post("/storage")]
async fn create_storage(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let storage: StorageCreateRequest = serde_json::from_str(&body)?;

    check_auth(CheckType::Admin, &db, &permissions).await?;

    db::storage::create(
        &db,
        &storage.name,
        storage.protected,
        storage.inventory_interval.and_then(|interval| {
            if interval < Interval::new(0, 1, 0) {
                None
            } else {
                Some(interval)
            }
        }),
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "storage",
    request_body = StorageUpdateRequest,
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
#[patch("/storage")]
async fn change_storage(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let storage: StorageUpdateRequest = serde_json::from_str(&body)?;

    check_auth(CheckType::Admin, &db, &permissions).await?;

    db::storage::change(
        &db,
        &storage.name,
        storage.new_name.as_deref(),
        storage.protected,
        storage.inventory_interval.and_then(|interval| {
            if interval < Interval::new(0, 1, 0) {
                None
            } else {
                Some(interval)
            }
        }),
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "storage",
    params(StorageDeleteQuery),
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
#[delete("/storage")]
async fn destroy_storage(
    query: web::Query<StorageDeleteQuery>,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    check_auth(CheckType::Admin, &db, &permissions).await?;
    db::storage::destroy(&db, &query.name).await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "storage",
    request_body = ContainerCreateRequest,
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
#[post("/container")]
async fn create_container(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let container: ContainerCreateRequest = serde_json::from_str(&body)?;

    check_auth(
        CheckType::Storage {
            storage: &container.storage,
            container: None,
        },
        &db,
        &permissions,
    )
    .await?;

    db::container::create(
        &db,
        &container.name,
        &container.storage,
        container.inventory_interval.and_then(|interval| {
            if interval < Interval::new(0, 1, 0) {
                None
            } else {
                Some(interval)
            }
        }),
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "storage",
    request_body = ContainerUpdateRequest,
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
#[patch("/container")]
async fn change_container(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let container: ContainerUpdateRequest = serde_json::from_str(&body)?;

    check_auth(
        CheckType::Storage {
            storage: &container.storage,
            container: Some(&container.name),
        },
        &db,
        &permissions,
    )
    .await?;

    db::container::change(
        &db,
        &container.name,
        container.new_name.as_deref(),
        &container.storage,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "storage",
    request_body = ContainerMoveRequest,
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
#[patch("/container/move")]
async fn move_container(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
    id: web::ReqData<String>,
) -> Result<HttpResponse, Error> {
    let container: ContainerMoveRequest = serde_json::from_str(&body)?;

    check_auth(
        CheckType::MoveContainer {
            container: &container.name,
            to_storage: &container.to_storage,
            from_storage: &container.from_storage,
        },
        &db,
        &permissions,
    )
    .await?;

    db::container::move_container(
        &db,
        &container.name,
        &container.from_storage,
        &container.to_storage,
        &id,
        container.merge,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "storage",
    params(ContainerDeleteQuery),
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
#[delete("/container")]
async fn destroy_container(
    query: web::Query<ContainerDeleteQuery>,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    check_auth(CheckType::Admin, &db, &permissions).await?;
    db::container::destroy(&db, &query.name, &query.storage).await?;
    Ok(HttpResponse::Ok().finish())
}
