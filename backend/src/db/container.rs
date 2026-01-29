use serde::Serialize;
use sqlx::{
    postgres::{types::PgInterval, PgQueryResult},
    Pool, Postgres,
};
use utoipa::ToSchema;

use crate::{db::{self, item::MinimalItem}, error::Error};

#[cfg(test)]
pub struct Container {
    pub name: String,
    pub storage: String,
    pub inventory_interval: Option<db::interval::Interval>,
}

/// List of containers at a storage location
#[derive(Debug, PartialEq, Serialize, ToSchema)]
pub struct ContainerStorage {
    /// The storages name
    name: String,
    /// List of containers
    containers: Vec<String>,
}

/// List of containers with items
#[derive(Debug, Serialize, PartialEq, ToSchema)]
pub struct ContainerItem {
    /// The containers name
    name: String,
    /// List of items
    items: Vec<MinimalItem>,
}

pub async fn get_all_containers_grouped_by_storage(
    db: &Pool<Postgres>,
    protected: &[String],
) -> Result<Vec<ContainerStorage>, sqlx::Error> {
    sqlx::query_as!(
        ContainerStorage,
        r#"
            SELECT storage.name, ARRAY_AGG(container.name) AS "containers!"
            FROM storage
            JOIN container ON storage.name = container.storage
            WHERE protected <> true OR LOWER(storage.name) IN (SELECT UNNEST($1::TEXT[]))
            GROUP BY storage.name
        "#,
        protected
    )
    .fetch_all(db)
    .await
}

struct Entry {
    name: String,
}

pub async fn get_all_containers_in_storage_with_items(
    db: &Pool<Postgres>,
    storage: &str,
) -> Result<Vec<ContainerItem>, sqlx::Error> {
    let containers = sqlx::query_as!(
        Entry,
        r#"
            SELECT name
            FROM container
            WHERE storage = $1
        "#,
        storage
    )
    .fetch_all(db)
    .await?;

    let mut result = Vec::new();

    for Entry { name: container } in containers {
        result.push(ContainerItem {
            name: container.clone(),
            items: db::item::get_all_in_storage_grouped_by_container_minimal(
                db, storage, &container,
            )
            .await?,
        });
    }

    Ok(result)
}

pub async fn create(
    db: &Pool<Postgres>,
    name: &str,
    storage: &str,
    inventory_interval: Option<PgInterval>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO container (name, storage, inventory_interval)
            VALUES ($1, $2, $3)
        "#,
        name,
        storage,
        inventory_interval
    )
    .execute(db)
    .await
}

pub async fn change(
    db: &Pool<Postgres>,
    name: &str,
    new_name: Option<&str>,
    storage: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    let new_name = if let Some(name) = new_name {
        name
    } else {
        name
    };

    sqlx::query!(
        r#"
            UPDATE container
            SET
                name = $2,
                storage = $3
            WHERE name = $1
        "#,
        name,
        new_name,
        storage,
    )
    .execute(db)
    .await
}

struct Item {
    item: String,
}

pub async fn move_container(
    db: &Pool<Postgres>,
    name: &str,
    from_storage: &str,
    to_storage: &str,
    id: &str,
    merge: bool,
) -> Result<(), Error> {
    let mut trans = db.begin().await?;

    let result = sqlx::query!(
        r#"
            UPDATE container
            SET storage = $1
            WHERE name = $2 AND storage = $3
        "#,
        to_storage,
        name,
        from_storage
    )
    .execute(&mut *trans)
    .await;

    if result.is_err() && merge {
        trans.rollback().await.unwrap();
        let mut trans = db.begin().await?;
        let items = sqlx::query_as!(
            Item,
            r#"
                SELECT item
                FROM stored_item
                WHERE storage = $1 AND container = $2
            "#,
            from_storage,
            name
        )
        .fetch_all(&mut *trans)
        .await?;

        for Item { item } in items {
            db::item::move_item(&mut trans, &item, None, from_storage, name, to_storage, name, id)
                .await?;
        }

        sqlx::query!(
            r#"
                DELETE FROM container
                WHERE storage = $1 AND name = $2
            "#,
            from_storage,
            name
        )
        .execute(&mut *trans)
        .await?;

        Ok(trans.commit().await?)
    } else if let Err(error) = result {
        trans.rollback().await?;
        Err(error.into())
    } else {
        Ok(trans.commit().await?)
    }
}

pub async fn destroy(
    db: &Pool<Postgres>,
    name: &str,
    storage: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            DELETE FROM container
            WHERE name = $1 AND storage = $2
        "#,
        name,
        storage
    )
    .execute(db)
    .await
}

#[cfg(test)]
mod test {
    use std::vec;

    use sqlx::{Pool, Postgres};

    use crate::db::{
        self,
        container::{Container, ContainerItem, ContainerStorage},
        interval::Interval,
        item::{BasicItem, BasicItemStorage, MinimalItem},
        OrderState,
    };

    #[sqlx::test]
    async fn get(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        let containers = super::get_all_containers_grouped_by_storage(&db, &Vec::new())
            .await
            .unwrap();

        assert_eq!(
            containers,
            vec![ContainerStorage {
                name: String::from("meta"),
                containers: vec![String::new()]
            }]
        )
    }

    #[sqlx::test]
    async fn get_more_than_one_container(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(&db, "Märkeslåda", "meta", None)
            .await
            .unwrap();

        let containers = super::get_all_containers_grouped_by_storage(&db, &Vec::new())
            .await
            .unwrap();

        assert_eq!(
            containers,
            vec![ContainerStorage {
                name: String::from("meta"),
                containers: vec![String::new(), String::from("Märkeslåda")]
            }]
        )
    }

    #[sqlx::test]
    async fn get_more_than_one_storage(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "ESCapen", false, None)
            .await
            .unwrap();

        let containers = super::get_all_containers_grouped_by_storage(&db, &Vec::new())
            .await
            .unwrap();

        assert_eq!(
            containers,
            vec![
                ContainerStorage {
                    name: String::from("ESCapen"),
                    containers: vec![String::new()]
                },
                ContainerStorage {
                    name: String::from("meta"),
                    containers: vec![String::new()]
                },
            ]
        )
    }

    #[sqlx::test]
    async fn dont_get_protected(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "spritis", true, None)
            .await
            .unwrap();

        let containers = super::get_all_containers_grouped_by_storage(&db, &Vec::new())
            .await
            .unwrap();

        assert_eq!(
            containers,
            vec![ContainerStorage {
                name: String::from("meta"),
                containers: vec![String::new()]
            },]
        )
    }

    #[sqlx::test]
    async fn get_protected_with_permission(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "spritis", true, None)
            .await
            .unwrap();

        let containers =
            super::get_all_containers_grouped_by_storage(&db, &vec![String::from("spritis")])
                .await
                .unwrap();

        assert_eq!(
            containers,
            vec![
                ContainerStorage {
                    name: String::from("spritis"),
                    containers: vec![String::new()]
                },
                ContainerStorage {
                    name: String::from("meta"),
                    containers: vec![String::new()]
                },
            ]
        )
    }

    #[sqlx::test]
    async fn get_detailed_tree(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        db::container::create(&db, "tejplåda", "meta", None)
            .await
            .unwrap();

        db::item::create(
            &db,
            "test",
            "meta",
            "",
            "pantsäck",
            None,
            None,
            3.0,
            Some("rullar"),
            None,
        )
        .await
        .unwrap();
        db::item::create(
            &db,
            "test",
            "meta",
            "",
            "ziptie",
            None,
            None,
            50.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();
        db::item::create(
            &db,
            "test",
            "meta",
            "tejplåda",
            "silvertejp",
            None,
            None,
            5.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();
        db::item::create(
            &db,
            "test",
            "meta",
            "tejplåda",
            "eltejp",
            None,
            None,
            5.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        let tree = super::get_all_containers_in_storage_with_items(&db, "meta")
            .await
            .unwrap();

        assert_eq!(
            tree,
            vec![
                ContainerItem {
                    name: String::from(""),
                    items: vec![
                        MinimalItem {
                            name: String::from("pantsäck"),
                            amount: 3.0,
                            unit: String::from("rullar"),
                            state: OrderState::None,
                            next_inventory: None
                        },
                        MinimalItem {
                            name: String::from("ziptie"),
                            amount: 50.0,
                            unit: String::from("st"),
                            state: OrderState::None,
                            next_inventory: None
                        }
                    ]
                },
                ContainerItem {
                    name: String::from("tejplåda"),
                    items: vec![
                        MinimalItem {
                            name: String::from("eltejp"),
                            amount: 5.0,
                            unit: String::from("st"),
                            state: OrderState::None,
                            next_inventory: None
                        },
                        MinimalItem {
                            name: String::from("silvertejp"),
                            amount: 5.0,
                            unit: String::from("st"),
                            state: OrderState::None,
                            next_inventory: None
                        },
                    ]
                },
            ]
        )
    }

    #[sqlx::test]
    async fn create(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(&db, "Märkeslåda", "meta", None)
            .await
            .unwrap();

        let container = sqlx::query_as!(
            Container,
            r#"
                SELECT name, storage, inventory_interval as "inventory_interval: Interval"
                FROM container
                WHERE name = 'Märkeslåda'
            "#,
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(container.name, "Märkeslåda");
        assert_eq!(container.storage, "meta");
        assert_eq!(container.inventory_interval, None);
    }

    #[sqlx::test]
    async fn crate_more_than_one(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();

        super::create(&db, "Märkeslåda", "meta", None)
            .await
            .unwrap();

        super::create(&db, "Snickarlåda", "meta", None)
            .await
            .unwrap();

        let containers = sqlx::query_as!(
            Container,
            r#"
                SELECT name, storage, inventory_interval as "inventory_interval: Interval"
                FROM container
                WHERE name <> ''
            "#,
        )
        .fetch_all(&db)
        .await
        .unwrap();

        assert_eq!(containers.len(), 2);
    }

    #[sqlx::test]
    async fn move_container(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "örådet", false, None)
            .await
            .unwrap();

        super::create(&db, "Märkeslåda", "meta", None)
            .await
            .unwrap();

        db::item::create(
            &db,
            "admin",
            "meta",
            "Märkeslåda",
            "tejp",
            None,
            None,
            5.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        super::move_container(&db, "Märkeslåda", "meta", "örådet", "test", false)
            .await
            .unwrap();

        let container = sqlx::query_as!(
            Container,
            r#"
                SELECT name, storage, inventory_interval as "inventory_interval: Interval"
                FROM container
                WHERE name = 'Märkeslåda'
            "#,
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(container.name, "Märkeslåda");
        assert_eq!(container.storage, "örådet");
        assert_eq!(container.inventory_interval, None);

        let item = db::item::get_all_filtered_basic(
            &db,
            None,
            None,
            None,
            None,
            None,
            None,
            &vec![String::from("meta"), String::from("örådet")],
        )
        .await
        .unwrap();

        assert_eq!(
            item,
            vec![BasicItem {
                name: String::from("tejp"),
                amount: 5.0,
                unit: String::from("st"),
                storage: vec![BasicItemStorage {
                    storage: String::from("örådet"),
                    container: String::from("Märkeslåda"),
                    state: OrderState::None
                }]
            }]
        )
    }

    #[sqlx::test]
    async fn move_conflict(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "örådet", false, None)
            .await
            .unwrap();

        super::create(&db, "Märkeslåda", "meta", None)
            .await
            .unwrap();

        super::create(&db, "Märkeslåda", "örådet", None)
            .await
            .unwrap();

        db::item::create(
            &db,
            "admin",
            "meta",
            "Märkeslåda",
            "tejp",
            None,
            None,
            5.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        assert!(
            super::move_container(&db, "Märkeslåda", "meta", "örådet", "test", false)
                .await
                .is_err()
        )
    }

    #[sqlx::test]
    async fn move_with_merge(db: Pool<Postgres>) {
        db::storage::create(&db, "meta", false, None).await.unwrap();
        db::storage::create(&db, "örådet", false, None)
            .await
            .unwrap();

        super::create(&db, "Märkeslåda", "meta", None)
            .await
            .unwrap();

        super::create(&db, "Märkeslåda", "örådet", None)
            .await
            .unwrap();

        db::item::create(
            &db,
            "admin",
            "meta",
            "Märkeslåda",
            "tejp",
            None,
            None,
            5.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        db::item::create(
            &db,
            "admin",
            "örådet",
            "Märkeslåda",
            "tejp",
            None,
            None,
            5.0,
            Some("st"),
            None,
        )
        .await
        .unwrap();

        super::move_container(&db, "Märkeslåda", "meta", "örådet", "test", true)
            .await
            .unwrap();

        let item = db::item::get_all_filtered_basic(
            &db,
            None,
            None,
            None,
            None,
            None,
            None,
            &vec![String::from("meta"), String::from("örådet")],
        )
        .await
        .unwrap();

        assert_eq!(
            item,
            vec![BasicItem {
                name: String::from("tejp"),
                amount: 10.0,
                unit: String::from("st"),
                storage: vec![BasicItemStorage {
                    storage: String::from("örådet"),
                    container: String::from("Märkeslåda"),
                    state: OrderState::None
                }]
            }]
        )
    }
}
