use actix_web::{delete, get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    Pool, Postgres,
};
use utoipa::{IntoParams, ToSchema};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    auth::{check_auth, types::HivePermission, CheckType},
    db::{self, shipment::ShipmentItem},
    error::Error,
};

#[derive(Debug, Serialize, ToSchema)]
struct ShipmentGetResponse {
    id: String,
    time_created: DateTime<Utc>,
    time_arive: DateTime<Utc>,
    items: Vec<ShipmentItem>,
}

#[derive(Debug, Deserialize, ToSchema)]
struct ShipmentCreateRequest {
    arrival_time: DateTime<Utc>,
    items: Vec<ShipmentItem>,
}

#[derive(Debug, Deserialize, IntoParams)]
struct ShipmentDeleteQuery {
    id: String,
}

pub(crate) fn config() -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut ServiceConfig| {
        cfg.service(get_shipment).service(create).service(destroy);
    }
}

#[utoipa::path(
    tag = "shipment",
    responses(
        (
            status = StatusCode::OK,
            body = Vec<ShipmentGetResponse>,
            description = "List of orders with info"
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
#[get("/shipment")]
pub(crate) async fn get_shipment(db: web::Data<Pool<Postgres>>) -> Result<HttpResponse, Error> {
    let shipments = db::shipment::get_all_orders(&db).await?;

    let shipments = shipments
        .into_iter()
        .map(|shipment| ShipmentGetResponse {
            id: shipment.id.to_string(),
            time_arive: shipment.time_arive.into(),
            time_created: shipment.time_created.into(),
            items: shipment.items,
        })
        .collect::<Vec<ShipmentGetResponse>>();

    Ok(HttpResponse::Ok().json(shipments))
}

#[utoipa::path(
    tag = "shipment",
    request_body = ShipmentCreateRequest,
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
#[post("/shipment")]
async fn create(
    body: String,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let shipment: ShipmentCreateRequest = serde_json::from_str(&body)?;

    check_auth(CheckType::Any, &db, &permissions).await?;

    db::shipment::create_order(&db, shipment.arrival_time.into(), shipment.items).await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    tag = "shipment",
    params(ShipmentDeleteQuery),
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
#[delete("/shipment")]
async fn destroy(
    query: web::Query<ShipmentDeleteQuery>,
    db: web::Data<Pool<Postgres>>,
    permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    check_auth(CheckType::Any, &db, &permissions).await?;

    db::shipment::delete(&db, Uuid::parse_str(&query.id)?).await?;

    Ok(HttpResponse::Ok().finish())
}
