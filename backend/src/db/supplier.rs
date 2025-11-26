use serde::Serialize;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use utoipa::ToSchema;

/// General info about a supplier
#[derive(Debug, PartialEq, Serialize, ToSchema)]
pub struct Supplier {
    /// The suppliers name
    name: String,
    /// Notes ex. order info
    notes: Option<String>,
    /// Username used to login to the suppliers website
    username: Option<String>,
    /// Password used to login to the suppliers website
    password: Option<String>,
    /// Link to the suppliers website
    link: Option<String>,
    /// Hive group this supplier is assosiated with
    #[serde(rename = "group")]
    pub mandate: String,
}

pub async fn get_count(db: &Pool<Postgres>) -> Result<Option<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT count(*)
            FROM supplier
        "#
    )
    .fetch_one(db)
    .await?
    .count)
}

pub async fn get_all_by_mandate(
    db: &Pool<Postgres>,
    mandates: &[String],
) -> Result<Vec<Supplier>, sqlx::Error> {
    sqlx::query_as!(
        Supplier,
        r#"
            SELECT *
            FROM supplier
            WHERE mandate IN (SELECT UNNEST($1::TEXT[]))
        "#,
        mandates
    )
    .fetch_all(db)
    .await
}

pub async fn get_by_name(db: &Pool<Postgres>, name: &str) -> Result<Supplier, sqlx::Error> {
    sqlx::query_as!(
        Supplier,
        r#"
            SELECT *
            FROM supplier
            WHERE name = $1
        "#,
        name
    )
    .fetch_one(db)
    .await
}

pub async fn create(
    db: &Pool<Postgres>,
    name: &str,
    notes: Option<&str>,
    username: Option<&str>,
    password: Option<&str>,
    link: Option<&str>,
    mandate: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO supplier (name, notes, username, password, link, mandate)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        name,
        notes,
        username,
        password,
        link,
        mandate
    )
    .execute(db)
    .await
}

pub async fn change(
    db: &Pool<Postgres>,
    name: &str,
    old_name: Option<&str>,
    notes: Option<&str>,
    username: Option<&str>,
    password: Option<&str>,
    link: Option<&str>,
    mandate: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    let old_name = if let Some(name) = old_name {
        name
    } else {
        name
    };

    sqlx::query!(
        r#"
            UPDATE supplier
            SET
                name = $2,
                notes = $3,
                username = $4,
                password = $5,
                link = $6,
                mandate = $7
            WHERE name = $1
        "#,
        old_name,
        name,
        notes,
        username,
        password,
        link,
        mandate
    )
    .execute(db)
    .await
}

pub async fn destroy(db: &Pool<Postgres>, name: &str) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            DELETE FROM supplier
            WHERE name = $1
        "#,
        name,
    )
    .execute(db)
    .await
}

#[cfg(test)]
mod test {
    use sqlx::{Pool, Postgres};

    use crate::db::supplier::Supplier;

    #[sqlx::test]
    async fn get(db: Pool<Postgres>) {
        super::create(&db, "ICA", None, None, None, None, "mister@metadorerna.se")
            .await
            .unwrap();

        let supplier = super::get_all_by_mandate(&db, &vec![String::from("mister@metadorerna.se")])
            .await
            .unwrap();

        assert_eq!(
            supplier,
            vec![Supplier {
                name: String::from("ICA"),
                notes: None,
                username: None,
                password: None,
                link: None,
                mandate: String::from("mister@metadorerna.se")
            }]
        )
    }

    #[sqlx::test]
    async fn get_when_missing_mandate(db: Pool<Postgres>) {
        super::create(&db, "ICA", None, None, None, None, "mister@metadorerna.se")
            .await
            .unwrap();

        let supplier = super::get_all_by_mandate(&db, &Vec::new()).await.unwrap();

        assert_eq!(supplier, Vec::new())
    }

    #[sqlx::test]
    async fn create(db: Pool<Postgres>) {
        super::create(&db, "ICA", None, None, None, None, "mister@metadorerna.se")
            .await
            .unwrap();

        let supplier = sqlx::query_as!(
            Supplier,
            r#"
                SELECT *
                FROM supplier
            "#,
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(
            supplier,
            Supplier {
                name: String::from("ICA"),
                notes: None,
                username: None,
                password: None,
                link: None,
                mandate: String::from("mister@metadorerna.se")
            }
        )
    }

    #[sqlx::test]
    async fn change(db: Pool<Postgres>) {
        super::create(&db, "ICA", None, None, None, None, "mister@metadorerna.se")
            .await
            .unwrap();

        super::change(
            &db,
            "IKEA",
            Some("ICA"),
            None,
            None,
            None,
            None,
            "mister@metadorerna.se",
        )
        .await
        .unwrap();

        let supplier = sqlx::query_as!(
            Supplier,
            r#"
                SELECT *
                FROM supplier
            "#,
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(
            supplier,
            Supplier {
                name: String::from("IKEA"),
                notes: None,
                username: None,
                password: None,
                link: None,
                mandate: String::from("mister@metadorerna.se")
            }
        )
    }

    #[sqlx::test]
    async fn delete(db: Pool<Postgres>) {
        super::create(&db, "ICA", None, None, None, None, "mister@metadorerna.se")
            .await
            .unwrap();

        super::destroy(&db, "ICA").await.unwrap();

        let supplier = super::get_all_by_mandate(&db, &vec![String::from("mister@metadorerna.se")])
            .await
            .unwrap();

        assert_eq!(supplier, vec![])
    }
}
