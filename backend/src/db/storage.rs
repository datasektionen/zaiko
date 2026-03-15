use serde::Serialize;
use sqlx::{
    postgres::{types::PgInterval, PgQueryResult},
    Pool, Postgres,
};
use utoipa::ToSchema;

use crate::db::{self, interval::Interval};

/// Info about a storage location
#[derive(Debug, PartialEq, Serialize, ToSchema, sqlx::FromRow)]
pub struct Storage {
    /// The suppliers name
    pub name: String,
    /// If the storage require a read permission
    pub protected: bool,
    /// The time between the storage should be inventoried
    pub inventory_interval: Option<Interval>,
}

pub async fn get_all_unprotected(db: &Pool<Postgres>) -> Result<Vec<Storage>, sqlx::Error> {
    sqlx::query_as!(
        Storage,
        r#"
            SELECT name, protected, inventory_interval as "inventory_interval: Interval"
            FROM storage
        "#,
    )
    .fetch_all(db)
    .await
}

pub async fn get_all(db: &Pool<Postgres>, access: &[String]) -> Result<Vec<Storage>, sqlx::Error> {
    sqlx::query_as!(
        Storage,
        r#"
            SELECT name, protected, inventory_interval as "inventory_interval: Interval"
            FROM storage
            WHERE protected <> true OR LOWER(name) IN (SELECT UNNEST($1::TEXT[]))
        "#,
        access
    )
    .fetch_all(db)
    .await
}

pub async fn create(
    db: &Pool<Postgres>,
    name: &str,
    protected: bool,
    inventory_interval: Option<Interval>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO storage (name, protected, inventory_interval)
            VALUES ($1, $2, $3)
        "#,
        name,
        protected,
        inventory_interval.map(Into::<PgInterval>::into)
    )
    .execute(db)
    .await?;

    db::container::create(db, "", name, None).await
}

pub async fn change(
    db: &Pool<Postgres>,
    name: &str,
    new_name: Option<&str>,
    protected: bool,
    inventory_interval: Option<Interval>,
) -> Result<PgQueryResult, sqlx::Error> {
    let new_name = if let Some(name) = new_name {
        name
    } else {
        name
    };

    sqlx::query!(
        r#"
            UPDATE storage
            SET
                name = $2,
                protected = $3,
                inventory_interval = $4
            WHERE name = $1
        "#,
        name,
        new_name,
        protected,
        inventory_interval.map(Into::<PgInterval>::into)
    )
    .execute(db)
    .await
}

pub async fn destroy(db: &Pool<Postgres>, name: &str) -> Result<(), sqlx::Error> {
    let mut db = db.begin().await?;

    sqlx::query!(
        r#"
            DELETE FROM container
            WHERE storage = $1 AND name = ''
        "#,
        name,
    )
    .execute(&mut *db)
    .await?;

    sqlx::query!(
        r#"
            DELETE FROM storage
            WHERE name = $1
        "#,
        name
    )
    .execute(&mut *db)
    .await?;

    db.commit().await
}

#[cfg(test)]
mod test {
    use sqlx::{postgres::types::PgInterval, Pool, Postgres};

    use crate::db::{self, interval::Interval, storage::Storage};

    #[sqlx::test]
    async fn get(db: Pool<Postgres>) {
        super::create(&db, "meta", false, None).await.unwrap();

        let storage = super::get_all(&db, &Vec::new()).await.unwrap();

        assert_eq!(
            storage,
            vec![Storage {
                name: String::from("meta"),
                protected: false,
                inventory_interval: None
            }]
        )
    }

    #[sqlx::test]
    async fn dont_get_protected(db: Pool<Postgres>) {
        super::create(&db, "meta", false, None).await.unwrap();
        super::create(&db, "ESCapen", false, None).await.unwrap();
        super::create(&db, "spritis", true, None).await.unwrap();

        let storages = super::get_all(&db, &Vec::new()).await.unwrap();

        assert_eq!(storages.len(), 2);
    }

    #[sqlx::test]
    async fn get_protected_with_permissiens(db: Pool<Postgres>) {
        super::create(&db, "meta", false, None).await.unwrap();
        super::create(&db, "ESCapen", false, None).await.unwrap();
        super::create(&db, "spritis", true, None).await.unwrap();

        let storages = super::get_all(&db, &vec![String::from("spritis")])
            .await
            .unwrap();

        assert_eq!(storages.len(), 3);
    }

    #[sqlx::test]
    async fn create(db: Pool<Postgres>) {
        super::create(&db, "meta", false, None).await.unwrap();

        let storage = sqlx::query_as!(
            super::Storage,
            r#"
                SELECT name, protected, inventory_interval as "inventory_interval: Interval"
                FROM storage
            "#
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(storage.name, "meta");
        assert_eq!(storage.protected, false);
        assert_eq!(storage.inventory_interval, None);
    }

    #[sqlx::test]
    async fn create_more_than_one(db: Pool<Postgres>) {
        super::create(&db, "meta", false, None).await.unwrap();
        super::create(&db, "ESCapen", false, None).await.unwrap();
        super::create(&db, "Spritis", false, None).await.unwrap();

        let storages = sqlx::query_as!(
            super::Storage,
            r#"
                SELECT name, protected, inventory_interval as "inventory_interval: Interval"
                FROM storage
            "#
        )
        .fetch_all(&db)
        .await
        .unwrap();

        assert_eq!(storages.len(), 3);
    }

    #[sqlx::test]
    async fn create_default_container(db: Pool<Postgres>) {
        super::create(&db, "meta", false, None).await.unwrap();

        let container = sqlx::query_as!(
            db::container::Container,
            r#"
            SELECT name, storage, inventory_interval as "inventory_interval: Interval"
            FROM container
        "#,
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(container.name, "");
        assert_eq!(container.storage, "meta");
        assert_eq!(container.inventory_interval, None);
    }

    #[sqlx::test]
    async fn update_name(db: Pool<Postgres>) {
        super::create(&db, "meta", false, None).await.unwrap();

        super::change(&db, "meta", Some("ESCapen"), false, None)
            .await
            .unwrap();

        let storage = sqlx::query_as!(
            super::Storage,
            r#"
                SELECT name, protected, inventory_interval as "inventory_interval: Interval"
                FROM storage
            "#
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(storage.name, "ESCapen");
        assert_eq!(storage.protected, false);
        assert_eq!(storage.inventory_interval, None);

        let container = sqlx::query_as!(
            db::container::Container,
            r#"
            SELECT name, storage, inventory_interval as "inventory_interval: Interval"
            FROM container
        "#,
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(container.name, "");
        assert_eq!(container.storage, "ESCapen");
        assert_eq!(container.inventory_interval, None);
    }

    #[sqlx::test]
    async fn update_values(db: Pool<Postgres>) {
        super::create(&db, "meta", false, None).await.unwrap();

        super::change(&db, "meta", None, true, Some(Interval::new(0, 1, 0)))
            .await
            .unwrap();

        let storage = sqlx::query_as!(
            super::Storage,
            r#"
                SELECT name, protected, inventory_interval as "inventory_interval: Interval"
                FROM storage
            "#
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(storage.name, "meta");
        assert_eq!(storage.protected, true);
        assert_eq!(storage.inventory_interval, Some(Interval::new(0, 1, 0)));
    }

    #[sqlx::test]
    async fn delete(db: Pool<Postgres>) {
        super::create(&db, "meta", false, None).await.unwrap();

        super::destroy(&db, "meta").await.unwrap();

        let storage = sqlx::query_as!(
            super::Storage,
            r#"
                SELECT name, protected, inventory_interval as "inventory_interval: Interval"
                FROM storage
            "#
        )
        .fetch_optional(&db)
        .await
        .unwrap();

        assert_eq!(storage, None);
    }
}
