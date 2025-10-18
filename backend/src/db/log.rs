use serde::Serialize;
use sqlx::{
    types::chrono::{DateTime, Utc},
    Pool, Postgres,
};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Log {
    State(StateLog),
    Move(MoveLog),
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StateLog {
    item: String,
    user: String,
    amount: f32,
    time: DateTime<Utc>,
    storage: String,
    container: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MoveLog {
    item: String,
    user: String,
    amount: f32,
    time: DateTime<Utc>,
    from_storage: String,
    from_container: String,
    to_storage: String,
    to_container: String,
}

pub async fn get_all_by_item(db: &Pool<Postgres>, item: &str) -> Result<Vec<Log>, sqlx::Error> {
    let state_log = sqlx::query_as!(
        StateLog,
        r#"
            SELECT item, user_ as "user", amount, time, storage, container
            FROM log
            WHERE item = $1
        "#,
        item
    )
    .fetch_all(db)
    .await?;

    let move_log = sqlx::query_as!(
        MoveLog,
        r#"
            SELECT item, user_ as "user", amount, time, from_storage, from_container, to_storage, to_container
            FROM move_log
            WHERE item = $1
        "#,
        item
    ).fetch_all(db).await?;

    Ok(state_log
        .into_iter()
        .map(|log| Log::State(log))
        .chain(move_log.into_iter().map(|log| Log::Move(log)))
        .collect())
}
