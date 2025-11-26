use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgQueryResult,
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    Pool, Postgres,
};
use utoipa::ToSchema;

// struct Shipment {
//     id: Uuid,
//     time_created: DateTime<Utc>,
//     time_arive: DateTime<Utc>,
// }
//
// struct ShipmentItem {
//     shipment: Uuid,
//     item: String,
//     amount: f32,
// }

#[derive(Debug)]
pub struct Shipment {
    pub id: Uuid,
    pub time_created: DateTime<Utc>,
    pub time_arive: DateTime<Utc>,
    pub items: Vec<ShipmentItem>,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "shipment_listing")]
pub struct ShipmentItem {
    item: String,
    amount: f32,
}

pub async fn get_all_orders(db: &Pool<Postgres>) -> Result<Vec<Shipment>, sqlx::Error> {
    Ok(sqlx::query_as!(
        Shipment,
        r#"
            SELECT
                id,
                time_created,
                time_arive,
                ARRAY(
                    SELECT (
                        shipment_item.item,
                        shipment_item.amount
                    )::shipment_listing
                    FROM shipment_item
                    WHERE shipment.id = shipment_item.shipment
                ) as "items!: Vec<ShipmentItem>"
            FROM shipment
        "#
    )
    .fetch_all(db)
    .await?)
}

pub async fn get_order_by_id(db: &Pool<Postgres>, id: Uuid) -> Result<Shipment, sqlx::Error> {
    Ok(sqlx::query_as!(
        Shipment,
        r#"
            SELECT
                id,
                time_created,
                time_arive,
                ARRAY(
                    SELECT (
                        shipment_item.item,
                        shipment_item.amount
                    )::shipment_listing
                    FROM shipment_item
                    WHERE shipment.id = shipment_item.shipment
                ) as "items!: Vec<ShipmentItem>"
            FROM shipment
            WHERE id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await?)
}

pub async fn create_order(
    db: &Pool<Postgres>,
    arival: DateTime<Utc>,
    items: Vec<ShipmentItem>,
) -> Result<(), sqlx::Error> {
    let id = sqlx::query!(
        r#"
            INSERT INTO shipment (id, time_created, time_arive)
            VALUES (gen_random_uuid(), CURRENT_TIMESTAMP, $1)
            RETURNING id
        "#,
        arival
    )
    .fetch_one(db)
    .await?
    .id;

    for ShipmentItem { item, amount } in items {
        sqlx::query!(
            r#"
                INSERT INTO shipment_item (shipment, item, amount)
                VALUES ($1, $2, $3)
            "#,
            id,
            item,
            amount
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

pub async fn delete(db: &Pool<Postgres>, id: Uuid) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            DELETE FROM shipment
            WHERE id = $1
        "#,
        id
    )
    .execute(db)
    .await
}
