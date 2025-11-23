use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::{IntoParams, ToSchema};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    auth::{
        check_auth,
        types::{Group, HivePermission},
        CheckType,
    },
    db::{self, item::BasicItem, supplier::Supplier},
    error::Error,
};

/// Info used to create a supplier
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct SupplierCreateRequest {
    /// The suppliers name
    name: String,
    /// Link to the suppliers website
    link: Option<String>,
    /// Notes ex. order info
    notes: Option<String>,
    /// Username used to login to the suppliers website
    username: Option<String>,
    /// Password used to login to the suppliers website
    password: Option<String>,
    /// Hive group this supplier is assosiated with
    group: String,
}

/// Info used to update a supplier
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct SupplierUpdateRequest {
    /// The name the supplier is supposed to have
    name: String,
    /// The name of the supplier before update (only if changeing)
    old_name: Option<String>,
    /// Link to the suppliers website
    link: Option<String>,
    /// Notes ex. order info
    notes: Option<String>,
    /// Username used to login to the suppliers website
    username: Option<String>,
    /// Password used to login to the suppliers website
    password: Option<String>,
    /// Hive group this supplier should be assosiated with
    group: String,
}

/// Info used to delete a supplier
#[derive(Debug, Deserialize, IntoParams)]
struct SupplierDeleteQuery {
    /// The suppliers name
    name: String,
}

pub(crate) fn config() -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut ServiceConfig| {
        cfg.service(get_suppliers)
            .service(create_supplier)
            .service(update_supplier)
            .service(delete_supplier);
    }
}

#[utoipa::path(
    tag = "supplier",
    responses(
        (
            status = StatusCode::OK,
            body = Vec<Supplier>,
            description = "List of suppliers with info"
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
#[get("/supplier")]
pub(crate) async fn get_suppliers(
    db: web::Data<Pool<Postgres>>,
    groups: web::ReqData<Vec<Group>>,
) -> Result<HttpResponse, Error> {
    let groups: Vec<String> = groups.iter().map(|group| group.0.clone()).collect();

    log::debug!("groups: {groups:?}");

    let suppliers = db::supplier::get_all_by_mandate(&db, &groups).await?;

    Ok(HttpResponse::Ok().json(suppliers))
}

#[utoipa::path(
    tag = "supplier",
    request_body = SupplierCreateRequest,
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
#[post("/supplier")]
async fn create_supplier(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
    groups: web::ReqData<Vec<Group>>,
) -> Result<HttpResponse, Error> {
    let supplier: SupplierCreateRequest = serde_json::from_str(&body)?;

    check_auth(
        CheckType::SupplierCreate {
            mandates: &groups,
            mandate: &supplier.group,
        },
        &db,
        &permissions,
    )
    .await?;

    db::supplier::create(
        &db,
        &supplier.name,
        supplier.notes.as_deref(),
        supplier.username.as_deref(),
        supplier.password.as_deref(),
        supplier.link.as_deref(),
        &supplier.group,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "supplier",
    request_body = SupplierUpdateRequest,
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
#[patch("/supplier")]
async fn update_supplier(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
    groups: web::ReqData<Vec<Group>>,
) -> Result<HttpResponse, Error> {
    let supplier: SupplierUpdateRequest = serde_json::from_str(&body)?;

    check_auth(
        CheckType::Supplier {
            mandates: &groups,
            name: &supplier.name,
        },
        &db,
        &permissions,
    )
    .await?;

    db::supplier::change(
        &db,
        &supplier.name,
        supplier.old_name.as_deref(),
        supplier.notes.as_deref(),
        supplier.username.as_deref(),
        supplier.password.as_deref(),
        supplier.link.as_deref(),
        &supplier.group,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "supplier",
    params(SupplierDeleteQuery),
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
#[delete("/supplier")]
async fn delete_supplier(
    query: web::Query<SupplierDeleteQuery>,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
    groups: web::ReqData<Vec<Group>>,
) -> Result<HttpResponse, Error> {
    check_auth(
        CheckType::Supplier {
            mandates: &groups,
            name: &query.name,
        },
        &db,
        &permissions,
    )
    .await?;

    db::supplier::destroy(&db, &query.name).await?;
    Ok(HttpResponse::Ok().finish())
}
