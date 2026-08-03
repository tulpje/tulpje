use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::{Json, chrono},
};
use tulpje_framework::Error;
use twilight_model::id::{
    Id,
    marker::{GuildMarker, MessageMarker},
};

use crate::db::DbId;

#[derive(Debug, FromRow)]
pub struct StateRow<T>
where
    T: Unpin + Sync + Send + Serialize + for<'de> Deserialize<'de>,
{
    pub id: i64,
    pub guild_id: DbId<GuildMarker>,
    pub message_id: DbId<MessageMarker>,
    pub state: Json<T>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get<T>(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    message_id: Id<MessageMarker>,
) -> Result<Option<StateRow<T>>, Error>
where
    T: Unpin + Sync + Send + Serialize + for<'de> Deserialize<'de>,
{
    Ok(sqlx::query_as!(
        StateRow,
        r#"
        SELECT
            id,
            guild_id,
            message_id,
            state AS "state: Json<T>",
            created_at,
            updated_at
        FROM
            interaction_state
        WHERE
            guild_id = $1
        AND
            message_id = $2
    "#,
        i64::from(DbId(guild_id)),
        i64::from(DbId(message_id))
    )
    .fetch_optional(db)
    .await?)
}

pub async fn set<T>(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    message_id: Id<MessageMarker>,
    state: T,
) -> Result<(), Error>
where
    T: Unpin + Sync + Send + Serialize + for<'de> Deserialize<'de>,
{
    sqlx::query!(
        r#"
        INSERT INTO interaction_state
            (guild_id, message_id, state)
        VALUES
            ($1, $2, $3)
        ON CONFLICT
            (guild_id, message_id)
        DO UPDATE SET
            state = $3,
            updated_at = NOW()
    "#,
        i64::from(DbId(guild_id)),
        i64::from(DbId(message_id)),
        Json(state) as _,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn delete(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    message_id: Id<MessageMarker>,
) -> Result<bool, Error> {
    Ok(sqlx::query!(
        r#"
        DELETE FROM
            interaction_state
        WHERE
            guild_id = $1
        AND
            message_id = $2
    "#,
        i64::from(DbId(guild_id)),
        i64::from(DbId(message_id))
    )
    .execute(db)
    .await?
    .rows_affected()
        > 0)
}
