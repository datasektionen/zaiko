use serde::Serialize;
use sqlx::{
    postgres::{types::PgInterval, PgQueryResult},
    types::chrono::{DateTime, Utc},
    Pool, Postgres, Transaction,
};
use utoipa::ToSchema;

use crate::db::interval::Interval;
use crate::db::OrderState;

pub struct Location {
    pub storage: String,
    pub container: String,
}

#[cfg(test)]
#[derive(Debug, PartialEq, sqlx::FromRow, Clone)]
struct Item {
    name: String,
    inventory_interval: Option<Interval>,
    unit: String,
}

#[cfg(test)]
#[derive(Debug, PartialEq)]
struct StoredItem {
    storage: String,
    container: String,
    item: String,
    min: Option<f32>,
    max: Option<f32>,
    amount: f32,
}

// struct SupplierItem {
//     supplier: String,
//     item: String,
//     link: Option<String>,
// }

/// An item with an amount lower than its order floor
#[derive(Serialize, ToSchema)]
pub struct ShortageItem {
    /// The items name
    name: String,
    /// The name of the storage where the item is located
    storage: String,
    /// The name of the container where the item is stored
    container: String,
    /// The number of item currently in storage
    amount: f32,
    /// The number of items to buy to reach the order ceiling
    amount_to_buy: Option<f32>,
    /// The unit that the amount is measured in
    unit: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema, PartialEq)]
pub struct DueStorage {
    name: String,
    containers: Option<Vec<DueContainer>>,
}

#[derive(Debug, Serialize, sqlx::FromRow, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "shortage_listing")]
pub struct DueContainer {
    name: String,
    items: Vec<DueItem>,
}

/// An item due to be inventoried
#[derive(Debug, Serialize, sqlx::FromRow, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "shortage_item")]
pub struct DueItem {
    // The items name
    name: String,
    // // The name of the storage where the item is located
    // storage: String,
    // // The name of the container where the item is stored
    // container: String,
    /// The unit that the amount is measured in
    unit: Option<String>,
    // The number of item currently in storage
    amount: f32,
}

/// An item ment to be viewed in the context of a tree representation of the db
#[derive(Debug, Serialize, PartialEq, ToSchema)]
pub struct MinimalItem {
    /// The items name
    pub name: String,
    /// The current amount
    pub amount: f32,
    /// The unit current is counted in
    pub unit: String,
    /// The current state of the item (good/need to order/etc)
    pub state: Option<OrderState>,
    /// The next date when the item needs to be inventoried
    pub next_inventory: Option<DateTime<Utc>>,
}

/// Basic item information
#[derive(Debug, PartialEq, Serialize, ToSchema)]
pub struct BasicItem {
    /// The items name
    pub name: String,
    /// The current total amount of the item accros all storages
    pub amount: Option<f32>,
    /// The unit the amount is counted in
    pub unit: String,
    /// List of storage locations containing the item with basic info
    pub storage: Option<Vec<BasicItemStorage>>,
}

/// Basic info about where an item is stored
#[derive(Debug, PartialEq, Serialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "storage_listing")]
pub struct BasicItemStorage {
    /// The storage where the item is stored
    pub storage: String,
    /// The container where the item is stored
    pub container: String,
    /// The state the item is in (if more is needed to ordered)
    pub state: OrderState,
}

/// Detailed item infarmation
#[derive(Debug, PartialEq, Serialize, ToSchema)]
pub struct DetailedItem {
    /// The items name
    pub name: String,
    /// The avrage weekly consumption over the last mounth
    pub avrage_consuption: Option<f32>,
    /// The maximum inventory interval of the item
    pub inventory_interval: Option<Interval>,
    /// The unit every amount is counted in
    pub unit: String,
    /// The storages that this item type is stored in
    pub storage: Option<Vec<StorageListing>>,
    /// The suppliers that the item is bought from
    pub supplier: Option<Vec<SupplierListing>>,
}

/// Detailed information about where an item is stored
#[derive(Debug, PartialEq, sqlx::Type, Serialize, ToSchema)]
pub struct StorageListing {
    /// The location where the item is stored
    storage: String,
    /// The container in which the item is stored
    container: String,
    /// The number of items currently in storage
    amount: f32,
    /// The order floor for this storage
    min: Option<f32>,
    /// The order ceiling for this storage
    max: Option<f32>,
    /// The state of the item (if more should be ordered)
    state: OrderState,
    /// The next time the item is to be inventoried
    next_inventory: Option<DateTime<Utc>>,
}

/// Information about the supplier of an item
#[derive(Debug, PartialEq, sqlx::Type, Serialize, ToSchema)]
pub struct SupplierListing {
    /// The name of the supplier
    name: String,
    /// A link to the item on the suppliers website
    link: Option<String>,
    /// If this is the prefered supplier of this item
    prfered: bool,
}

pub async fn get_count(db: &Pool<Postgres>) -> Result<Option<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT count(*)
            FROM item
        "#
    )
    .fetch_one(db)
    .await?
    .count)
}

pub async fn get_location(db: &Pool<Postgres>, item: &str) -> Result<Vec<Location>, sqlx::Error> {
    sqlx::query_as!(
        Location,
        r#"
            SELECT storage, container
            FROM stored_item
            WHERE item = $1
        "#,
        item
    )
    .fetch_all(db)
    .await
}

pub async fn get_all_in_storage_grouped_by_container_minimal(
    db: &Pool<Postgres>,
    storage: &str,
    container: &str,
) -> Result<Vec<MinimalItem>, sqlx::Error> {
    sqlx::query_as!(
        MinimalItem,
        r#"
            SELECT
                item.name,
                stored_item.amount,
                item.unit,
                STATE(
                    stored_item.amount,
                    stored_item.min,
                    stored_item.max,
                    EXISTS(
                        SELECT 1
                        FROM shipment_item
                        WHERE shipment_item.item = item.name
                    )
                ) as "state: OrderState",
                MAX(log.time) + 
                        LEAST(
                            storage.inventory_interval,
                            container.inventory_interval,
                            item.inventory_interval
                        ) as "next_inventory"
            FROM stored_item
            JOIN item ON item.name = stored_item.item
            JOIN log ON log.item = stored_item.item
            JOIN container ON container.name = stored_item.container
            JOIN storage ON storage.name = stored_item.storage
            LEFT JOIN shipment_item ON shipment_item.item = stored_item.item
            WHERE stored_item.storage = $1 AND stored_item.container = $2
            GROUP BY item.name, stored_item.amount, stored_item.min, stored_item.max, storage.inventory_interval, container.inventory_interval
        "#,
        storage,
        container
    ).fetch_all(db).await
}

pub async fn get_all_filtered_basic(
    db: &Pool<Postgres>,
    name: Option<&str>,
    storage: Option<&str>,
    container: Option<&str>,
    supplier: Option<&str>,
    min: Option<f32>,
    max: Option<f32>,
) -> Result<Vec<BasicItem>, sqlx::Error> {
    sqlx::query_as!(
        BasicItem,
        r#"
            SELECT
                item.name,
                SUM(stored_item.amount) as "amount",
                item.unit,
                ARRAY(
                    SELECT (
                        stored_item.storage,
                        stored_item.container,
                        STATE(
                            stored_item.amount,
                            stored_item.min,
                            stored_item.max,
                            EXISTS(
                                SELECT 1
                                FROM shipment_item
                                WHERE shipment_item.item = item.name
                            )
                        ))::storage_listing_basic
                    FROM stored_item
                    JOIN storage ON stored_item.storage = storage.name
                    WHERE stored_item.item = item.name AND storage.protected <> true
                ) AS "storage: Vec<BasicItemStorage>"
            FROM item
            JOIN stored_item ON stored_item.item = item.name
            LEFT JOIN supplier_item ON supplier_item.item = item.name
            WHERE
                ($1::TEXT IS NULL OR levenshtein(item.name, $1) <= 5) AND
                ($2::TEXT IS NULL OR storage = $2) AND
                ($3::TEXT IS NULL OR container = $3) AND
                ($4::TEXT IS NULL OR supplier_item.supplier = $4) AND
                ($5::REAL IS NULL OR stored_item.amount >= $5) AND
                ($6::REAL IS NULL OR stored_item.amount <= $6)
            GROUP BY item.name
        "#,
        name,
        storage,
        container,
        supplier,
        min,
        max
    )
    .fetch_all(db)
    .await
}

pub async fn get_item_by_name_detailed(
    db: &Pool<Postgres>,
    name: &str,
) -> Result<DetailedItem, sqlx::Error> {
    sqlx::query_as!(
        DetailedItem,
        r#"
            SELECT
                item.name,
                SUM(avrage_consuption.change) AS "avrage_consuption",
                item.unit,
                item.inventory_interval as "inventory_interval: Interval",
                ARRAY(
                    SELECT
                        (
                            stored_item.storage,
                            stored_item.container,
                            stored_item.amount,
                            stored_item.min,
                            stored_item.max,
                            STATE(
                                stored_item.amount,
                                stored_item.min,
                                stored_item.max,
                                EXISTS(
                                    SELECT 1
                                    FROM shipment_item
                                    WHERE shipment_item.item = stored_item.item
                                )
                            ),
                            MAX(log.time) + 
                                    LEAST(
                                        storage.inventory_interval,
                                        item.inventory_interval
                                    )
                        )::storage_listing
                    FROM stored_item
                    JOIN storage ON stored_item.storage = storage.name
                    LEFT JOIN log ON stored_item.item = log.item
                        AND stored_item.storage = log.storage
                        AND stored_item.container = log.container
                    WHERE stored_item.item = $1 AND storage.protected <> true
                    GROUP BY stored_item.item, stored_item.storage, stored_item.container, storage.inventory_interval
                ) AS "storage: Vec<StorageListing>",
                ARRAY(
                    SELECT (
                        supplier,
                        link,
                        prefered
                    )::supplier_listing
                    FROM supplier_item
                    WHERE item = $1
                ) AS "supplier: Vec<SupplierListing>"
            FROM item
            LEFT JOIN log ON item.name = log.item
            LEFT JOIN avrage_consuption ON avrage_consuption.item = item.name
            WHERE item.name = $1
            GROUP BY item.name
        "#,
        name,
    )
    .fetch_one(db)
    .await
}

pub async fn get_shortage(
    db: &Pool<Postgres>,
    protected: &[String],
) -> Result<Vec<ShortageItem>, sqlx::Error> {
    sqlx::query_as!(
        ShortageItem,
        r#"
            SELECT item.name as name, storage, container, amount, (max - amount) AS "amount_to_buy", unit
            FROM stored_item
            JOIN storage ON storage.name = stored_item.storage
            JOIN item ON stored_item.item = item.name
            WHERE (protected <> true OR storage.name IN (SELECT unnest($1::text[]))) AND amount <= min
        "#,
        protected
    )
    .fetch_all(db)
    .await
}

pub async fn get_shortage_count(db: &Pool<Postgres>) -> Result<Option<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
        SELECT count(*)
        FROM stored_item
        WHERE amount <= min
    "#
    )
    .fetch_one(db)
    .await?
    .count)
}

pub async fn items_due(
    db: &Pool<Postgres>,
    storages: &[String],
) -> Result<Vec<DueStorage>, sqlx::Error> {
    Ok(sqlx::query_as!(
        DueStorage,
        r#"
            SELECT
                name,
                ARRAY(
                    SELECT
                        (name,
                        ARRAY(
                            SELECT (name, unit, amount)::shortage_item
                            FROM stored_item
                            JOIN item ON item.name = stored_item.item
                            JOIN next_inventory ON next_inventory.item = stored_item.item
                                AND next_inventory.time < CURRENT_TIMESTAMP
                            WHERE
                                stored_item.container = container.name
                                AND stored_item.storage = storage.name
                        ))::shortage_listing
                    FROM container
                    WHERE container.storage = storage.name
                ) AS "containers!: Vec<DueContainer>"
            FROM storage
            WHERE storage.name IN (SELECT unnest($1::text[]))
        "#,
        storages
    )
    .fetch_all(db)
    .await?)
}

pub async fn create(
    db: &Pool<Postgres>,
    id: &str,
    storage: &str,
    container: &str,
    item: &str,
    min: Option<f32>,
    max: Option<f32>,
    amount: f32,
    unit: Option<&str>,
    inventory_interval: Option<Interval>,
) -> Result<(), sqlx::Error> {
    let mut db = db.begin().await?;
    if let Some(unit) = unit {
        sqlx::query!(
            r#"
            INSERT INTO item (name, unit, inventory_interval)
            VALUES ($1, $2, $3)
            ON CONFLICT (name) DO NOTHING
        "#,
            item,
            unit,
            inventory_interval.map(Into::<PgInterval>::into)
        )
        .execute(&mut *db)
        .await?;
    } else {
        sqlx::query!(
            r#"
            INSERT INTO item (name, inventory_interval)
            VALUES ($1, $2)
            ON CONFLICT (name) DO NOTHING
        "#,
            item,
            inventory_interval.map(Into::<PgInterval>::into)
        )
        .execute(&mut *db)
        .await?;
    }

    sqlx::query!(
        r#"
            INSERT INTO stored_item (
                storage,
                container,
                item,
                min,
                max,
                amount
            )
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        storage,
        container,
        item,
        min,
        max,
        amount,
    )
    .execute(&mut *db)
    .await?;

    sqlx::query!(
        r#"
            INSERT INTO log (
                item,
                storage,
                container,
                amount,
                user_
            )
            VALUES ($1, $2, $3, $4, $5)
        "#,
        item,
        storage,
        container,
        amount,
        id
    )
    .execute(&mut *db)
    .await?;

    db.commit().await
}

pub async fn add_supplier(
    db: &Pool<Postgres>,
    supplier: &str,
    item: &str,
    link: Option<&str>,
    prefered: bool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO supplier_item (supplier, item, link, prefered)
                VALUES ($1, $2, $3, $4)
            "#,
        supplier,
        item,
        link,
        prefered
    )
    .execute(db)
    .await
}

pub async fn change(
    db: &Pool<Postgres>,
    name: &str,
    new_name: Option<&str>,
    unit: &str,
    inventory_interval: Option<Interval>,
) -> Result<PgQueryResult, sqlx::Error> {
    let new_name = if let Some(name) = new_name {
        name
    } else {
        name
    };

    sqlx::query!(
        r#"
            UPDATE item
            SET 
                name = $2,
                unit = $3,
                inventory_interval = $4
            WHERE name = $1
        "#,
        name,
        new_name,
        unit,
        inventory_interval.map(Into::<PgInterval>::into)
    )
    .execute(db)
    .await
}

pub async fn change_stored_item(
    db: &Pool<Postgres>,
    name: &str,
    amount: f32,
    min: Option<f32>,
    max: Option<f32>,
    storage: &str,
    new_storage: Option<&str>,
    container: &str,
    new_container: Option<&str>,
    id: &str,
) -> Result<(), sqlx::Error> {
    let mut db = db.begin().await?;

    let old_amount = sqlx::query_scalar!(
        r#"
            SELECT amount
            FROM stored_item
            WHERE item = $1 AND storage = $2 AND container = $3
        "#,
        name,
        storage,
        container
    )
    .fetch_one(&mut *db)
    .await?;

    let new_storage = if let Some(storage) = new_storage {
        storage
    } else {
        storage
    };

    let new_container = if let Some(container) = new_container {
        container
    } else {
        container
    };

    sqlx::query!(
        r#"
            UPDATE stored_item
            SET
                storage = $1,
                container = $2,
                amount = $3,
                min = $4,
                max = $5
            WHERE item = $6 AND storage = $7 AND container = $8
        "#,
        new_storage,
        new_container,
        amount,
        min,
        max,
        name,
        storage,
        container
    )
    .execute(&mut *db)
    .await?;

    if old_amount != amount {
        sqlx::query!(
            r#"
            INSERT INTO log (
                item,
                storage,
                container,
                amount,
                user_
            )
            VALUES ($1, $2, $3, $4, $5)
        "#,
            name,
            new_storage,
            new_container,
            amount,
            id
        )
        .execute(&mut *db)
        .await?;
    }

    db.commit().await
}

pub async fn move_item(
    db: &mut Transaction<'static, Postgres>,
    item: &str,
    from_storage: &str,
    from_container: &str,
    to_storage: &str,
    to_container: &str,
    id: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    let amount = sqlx::query_scalar!(
        r#"
            SELECT amount
            FROM stored_item
            WHERE item = $1 AND storage = $2 AND container = $3
        "#,
        item,
        from_storage,
        from_container
    )
    .fetch_one(&mut **db)
    .await?;

    if sqlx::query!(
        r#"
            SELECT (1) as "id"
            FROM stored_item
            WHERE
                item = $1 AND
                storage = $2 AND
                container = $3
        "#,
        item,
        to_storage,
        to_container
    )
    .fetch_optional(&mut **db)
    .await?
    .is_some()
    {
        sqlx::query!(
            r#"
                UPDATE stored_item
                SET amount =
                    amount + (
                        SELECT amount
                        FROM stored_item
                        WHERE item = $1 AND storage = $2 AND container = $3
                    )
                WHERE
                    item = $1 AND
                    storage = $4 AND
                    container = $5
            "#,
            item,
            from_storage,
            from_container,
            to_storage,
            to_container
        )
        .execute(&mut **db)
        .await?;

        sqlx::query!(
            r#"
                DELETE FROM stored_item
                WHERE item = $1 AND storage = $2 AND container = $3
            "#,
            item,
            from_storage,
            from_container
        )
        .execute(&mut **db)
        .await?;
    } else {
        let result = sqlx::query!(
            r#"
                UPDATE stored_item
                SET storage = $1, container = $2
                WHERE
                    item = $3 AND
                    storage = $4 AND
                    container = $5
            "#,
            to_storage,
            to_container,
            item,
            from_storage,
            from_container
        )
        .execute(&mut **db)
        .await?;

        if result.rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound);
        }
    }

    sqlx::query!(
        r#"
            INSERT INTO move_log (
                item,
                from_storage,
                from_container,
                to_storage,
                to_container,
                amount,
                user_
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        item,
        from_storage,
        from_container,
        to_storage,
        to_container,
        amount,
        id
    )
    .execute(&mut **db)
    .await
}

pub async fn update_amount_in_transaction(
    db: &mut Transaction<'static, Postgres>,
    id: &str,
    item: &str,
    storage: &str,
    container: &str,
    amount: f32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE stored_item
            SET amount = $1 
            WHERE item = $2 AND storage = $3 AND container = $4
        "#,
        amount,
        item,
        storage,
        container
    )
    .execute(&mut **db)
    .await?;

    sqlx::query!(
        r#"
            INSERT INTO log (
                item,
                storage,
                container,
                amount,
                user_
            )
            VALUES ($1, $2, $3, $4, $5)
        "#,
        item,
        storage,
        container,
        amount,
        id
    )
    .execute(&mut **db)
    .await
}

pub async fn delete_supplier(
    db: &Pool<Postgres>,
    item: &str,
    supplier: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            DELETE FROM supplier_item
            WHERE item = $1 AND supplier = $2
        "#,
        item,
        supplier
    )
    .execute(db)
    .await
}

pub async fn delete(
    db: &Pool<Postgres>,
    storage: &str,
    container: &str,
    item: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    let result = sqlx::query!(
        r#"
            DELETE FROM stored_item
            WHERE item = $1 AND storage = $2 AND container = $3
        "#,
        item,
        storage,
        container
    )
    .execute(db)
    .await?;

    if sqlx::query!(
        r#"
            SELECT item
            FROM stored_item
            WHERE item = $1
        "#,
        item
    )
    .fetch_optional(db)
    .await?
    .is_none()
    {
        sqlx::query!(
            r#"
                DELETE FROM item
                WHERE name = $1
            "#,
            item
        )
        .execute(db)
        .await
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use sqlx::{Pool, Postgres};

    use crate::db::{
        self,
        interval::Interval,
        item::{
            get_all_filtered_basic, BasicItem, BasicItemStorage, DetailedItem, DueContainer,
            DueItem, DueStorage, Item, MinimalItem, StorageListing, StoredItem,
        },
        OrderState,
    };

    #[sqlx::test]
    async fn get_minimal(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            None,
            None,
            5.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let minimal = super::get_all_in_storage_grouped_by_container_minimal(&db, "meta", "")
            .await
            .unwrap();

        assert_eq!(
            minimal,
            vec![MinimalItem {
                name: String::from("tejp"),
                amount: 5.0,
                unit: String::from("st"),
                state: Some(OrderState::None),
                next_inventory: None
            }]
        )
    }

    #[sqlx::test]
    async fn get_basic(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let item = super::get_all_filtered_basic(&db, None, None, None, None, None, None)
            .await
            .unwrap();

        assert_eq!(
            item,
            vec![BasicItem {
                name: String::from("tejp"),
                amount: Some(7.0),
                unit: String::from("st"),
                storage: Some(vec![BasicItemStorage {
                    storage: String::from("meta"),
                    container: String::new(),
                    state: OrderState::Good
                }])
            }]
        )
    }

    #[sqlx::test]
    async fn get_basic_filtered_by_name(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let item = super::get_all_filtered_basic(&db, Some("tejp"), None, None, None, None, None)
            .await
            .unwrap();

        assert_eq!(
            item,
            vec![BasicItem {
                name: String::from("tejp"),
                amount: Some(7.0),
                unit: String::from("st"),
                storage: Some(vec![BasicItemStorage {
                    storage: String::from("meta"),
                    container: String::from(""),
                    state: OrderState::Good,
                }])
            }]
        );

        let item =
            super::get_all_filtered_basic(&db, Some("hammare"), None, None, None, None, None)
                .await
                .unwrap();

        assert_eq!(item, vec![]);
    }

    #[sqlx::test]
    async fn get_basic_filtered_by_storage(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let item = super::get_all_filtered_basic(&db, None, Some("meta"), None, None, None, None)
            .await
            .unwrap();

        assert_eq!(
            item,
            vec![BasicItem {
                name: String::from("tejp"),
                amount: Some(7.0),
                unit: String::from("st"),
                storage: Some(vec![BasicItemStorage {
                    storage: String::from("meta"),
                    container: String::from(""),
                    state: OrderState::Good,
                }])
            }]
        );

        let item = super::get_all_filtered_basic(&db, None, Some("örådet"), None, None, None, None)
            .await
            .unwrap();

        assert_eq!(item, vec![]);
    }

    #[sqlx::test]
    async fn get_basic_filtered_by_container(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let item = super::get_all_filtered_basic(&db, None, None, Some(""), None, None, None)
            .await
            .unwrap();

        assert_eq!(
            item,
            vec![BasicItem {
                name: String::from("tejp"),
                amount: Some(7.0),
                unit: String::from("st"),
                storage: Some(vec![BasicItemStorage {
                    storage: String::from("meta"),
                    container: String::from(""),
                    state: OrderState::Good,
                }])
            }]
        );

        let item =
            super::get_all_filtered_basic(&db, None, None, Some("snickarlåda"), None, None, None)
                .await
                .unwrap();

        assert_eq!(item, vec![]);
    }

    #[sqlx::test]
    async fn get_basic_filtered_by_amount(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let item = super::get_all_filtered_basic(&db, None, None, None, None, Some(6.0), Some(8.0))
            .await
            .unwrap();

        assert_eq!(
            item,
            vec![BasicItem {
                name: String::from("tejp"),
                amount: Some(7.0),
                unit: String::from("st"),
                storage: Some(vec![BasicItemStorage {
                    storage: String::from("meta"),
                    container: String::from(""),
                    state: OrderState::Good,
                }])
            }]
        );

        let item =
            super::get_all_filtered_basic(&db, None, None, None, None, Some(8.0), Some(11.0))
                .await
                .unwrap();

        assert_eq!(item, vec![]);
    }

    #[sqlx::test]
    async fn get_basic_multiple_storages(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "örådet", false, None)
            .await
            .unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        super::create(
            &db,
            "test",
            "örådet",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let item = super::get_all_filtered_basic(&db, None, None, None, None, None, None)
            .await
            .unwrap();

        assert_eq!(
            item,
            vec![BasicItem {
                name: String::from("tejp"),
                amount: Some(14.0),
                unit: String::from("st"),
                storage: Some(vec![
                    BasicItemStorage {
                        storage: String::from("meta"),
                        container: String::new(),
                        state: OrderState::Good
                    },
                    BasicItemStorage {
                        storage: String::from("örådet"),
                        container: String::new(),
                        state: OrderState::Good
                    }
                ])
            }]
        )
    }

    #[sqlx::test]
    async fn get_detailed(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let item = super::get_item_by_name_detailed(&db, "tejp").await.unwrap();

        assert_eq!(
            item,
            DetailedItem {
                name: String::from("tejp"),
                avrage_consuption: None,
                unit: String::from("st"),
                inventory_interval: None,
                storage: Some(vec![StorageListing {
                    storage: String::from("meta"),
                    container: String::from(""),
                    amount: 7.0,
                    min: Some(5.0),
                    max: Some(10.0),
                    state: OrderState::Good,
                    next_inventory: None
                }]),
                supplier: Some(vec![])
            }
        )
    }

    #[sqlx::test]
    async fn get_detailed_multiple_storages(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "örådet", false, None)
            .await
            .unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        super::create(
            &db,
            "test",
            "örådet",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let item = super::get_item_by_name_detailed(&db, "tejp").await.unwrap();

        assert_eq!(
            item,
            DetailedItem {
                name: String::from("tejp"),
                avrage_consuption: None,
                unit: String::from("st"),
                inventory_interval: None,
                storage: Some(vec![
                    StorageListing {
                        storage: String::from("meta"),
                        container: String::from(""),
                        amount: 7.0,
                        min: Some(5.0),
                        max: Some(10.0),
                        state: OrderState::Good,
                        next_inventory: None
                    },
                    StorageListing {
                        storage: String::from("örådet"),
                        container: String::new(),
                        amount: 7.0,
                        min: Some(5.0),
                        max: Some(10.0),
                        state: OrderState::Good,
                        next_inventory: None,
                    }
                ]),
                supplier: Some(vec![])
            }
        )
    }

    #[sqlx::test]
    async fn create(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            None,
            None,
        )
        .await
        .unwrap();

        let item = sqlx::query_as!(
            Item,
            r#"
                SELECT name, unit, inventory_interval as "inventory_interval: Interval"
                FROM item
            "#
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(
            item,
            Item {
                name: String::from("tejp"),
                inventory_interval: None,
                unit: String::from("st")
            }
        );

        let stored_item = sqlx::query_as!(
            StoredItem,
            r#"
                SELECT *
                FROM stored_item
            "#
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(
            stored_item,
            StoredItem {
                item: String::from("tejp"),
                storage: String::from("meta"),
                container: String::new(),
                min: Some(5.0),
                max: Some(10.0),
                amount: 7.0
            }
        )
    }

    #[sqlx::test]
    async fn change(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        super::change(&db, "tejp", Some("silvertejp"), "rullar", None)
            .await
            .unwrap();

        let item = get_all_filtered_basic(&db, Some("silvertejp"), None, None, None, None, None)
            .await
            .unwrap();

        assert_eq!(
            item,
            vec![BasicItem {
                name: String::from("silvertejp"),
                amount: Some(7.0),
                unit: String::from("rullar"),
                storage: Some(vec![BasicItemStorage {
                    storage: String::from("meta"),
                    container: String::from(""),
                    state: OrderState::Good
                }])
            }]
        )
    }

    #[sqlx::test]
    async fn change_stored_item(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        db::container::create(&db, "tejplåda", "meta", None)
            .await
            .unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        super::change_stored_item(
            &db,
            "tejp",
            8.0,
            Some(6.0),
            Some(11.0),
            "meta",
            None,
            "",
            Some("tejplåda"),
            "test",
        )
        .await
        .unwrap();

        let item = super::get_item_by_name_detailed(&db, "tejp").await.unwrap();

        assert_eq!(
            item,
            DetailedItem {
                name: String::from("tejp"),
                avrage_consuption: None,
                unit: String::from("st"),
                inventory_interval: None,
                storage: Some(vec![StorageListing {
                    storage: String::from("meta"),
                    container: String::from("tejplåda"),
                    amount: 8.0,
                    min: Some(6.0),
                    max: Some(11.0),
                    state: OrderState::Good,
                    next_inventory: None
                }]),
                supplier: Some(vec![])
            }
        )
    }

    #[sqlx::test]
    async fn change_stored_item_destructive(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        db::container::create(&db, "tejplåda", "meta", None)
            .await
            .unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "tejplåda",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        assert!(super::change_stored_item(
            &db,
            "tejp",
            8.0,
            Some(6.0),
            Some(11.0),
            "meta",
            None,
            "",
            Some("tejplåda"),
            "test",
        )
        .await
        .is_err());
    }

    #[sqlx::test]
    async fn move_item(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "örådet", false, None)
            .await
            .unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let mut trans = db.begin().await.unwrap();

        super::move_item(&mut trans, "tejp", "meta", "", "örådet", "", "test")
            .await
            .unwrap();

        trans.commit().await.unwrap();

        let stored_item = sqlx::query_as!(
            StoredItem,
            r#"
                SELECT *
                FROM stored_item
            "#
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(
            stored_item,
            StoredItem {
                item: String::from("tejp"),
                storage: String::from("örådet"),
                container: String::new(),
                min: Some(5.0),
                max: Some(10.0),
                amount: 7.0
            }
        )
    }

    #[sqlx::test]
    async fn move_to_existing(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "örådet", false, None)
            .await
            .unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        super::create(
            &db,
            "test",
            "örådet",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let mut trans = db.begin().await.unwrap();

        println!(
            "{:?}",
            super::move_item(&mut trans, "tejp", "meta", "", "örådet", "", "test")
                .await
                .unwrap()
        );

        trans.commit().await.unwrap();

        let stored_item = sqlx::query_as!(
            StoredItem,
            r#"
                SELECT *
                FROM stored_item
            "#
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(
            stored_item,
            StoredItem {
                item: String::from("tejp"),
                storage: String::from("örådet"),
                container: String::new(),
                min: Some(5.0),
                max: Some(10.0),
                amount: 14.0
            }
        )
    }

    #[sqlx::test]
    async fn due_items(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "örådet", false, None)
            .await
            .unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "tejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            Some(Interval {
                months: 0,
                days: 7,
                microseconds: 0,
            }),
        )
        .await
        .unwrap();

        sqlx::query!(
            r#"
                UPDATE log
                    SET time = CURRENT_TIMESTAMP - INTERVAL '1 month'
                WHERE item = 'tejp' AND storage = 'meta' AND container = ''
            "#
        )
        .execute(&db)
        .await
        .unwrap();

        super::create(
            &db,
            "test",
            "meta",
            "",
            "eltejp",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            Some(Interval {
                months: 0,
                days: 7,
                microseconds: 0,
            }),
        )
        .await
        .unwrap();

        super::create(
            &db,
            "test",
            "örådet",
            "",
            "tändvätska",
            Some(5.0),
            Some(10.0),
            7.0,
            Some("st"),
            Some(Interval {
                months: 0,
                days: 7,
                microseconds: 0,
            }),
        )
        .await
        .unwrap();

        sqlx::query!(
            r#"
                UPDATE log
                    SET time = CURRENT_TIMESTAMP - INTERVAL '1 month'
                WHERE item = 'tändvätska' AND storage = 'örådet' AND container = ''
            "#
        )
        .execute(&db)
        .await
        .unwrap();

        let shortage = super::items_due(&db, &vec![String::from("meta"), String::from("örådet")])
            .await
            .unwrap();

        assert_eq!(
            shortage,
            vec![
                DueStorage {
                    name: String::from("meta"),
                    containers: Some(vec![DueContainer {
                        name: String::from(""),
                        items: vec![DueItem {
                            name: String::from("tejp"),
                            unit: Some(String::from("st")),
                            amount: 7.0
                        },]
                    }])
                },
                DueStorage {
                    name: String::from("örådet"),
                    containers: Some(vec![DueContainer {
                        name: String::from(""),
                        items: vec![DueItem {
                            name: String::from("tändvätska"),
                            unit: Some(String::from("st")),
                            amount: 7.0
                        }]
                    }])
                }
            ]
        )
    }
}
