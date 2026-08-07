use serde::{Deserialize, Serialize};
use sqlx::{
    PgPool,
    prelude::FromRow,
    types::{Json, chrono},
};
use tulpje_framework::Error;
use twilight_model::id::{Id, marker::GuildMarker};

use crate::db::DbId;

#[derive(Debug, FromRow)]
pub struct GuildSettings<T>
where
    T: Unpin + Sync + Send + Serialize + for<'de> Deserialize<'de>,
{
    pub id: i64,
    pub guild_id: DbId<GuildMarker>,
    pub module: String,
    pub scope: String,
    pub data: Json<T>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct GuildSettingsResolver {
    guild_id: Id<GuildMarker>,
    module: String,
    scope: String,
}

impl GuildSettingsResolver {
    pub fn new(
        guild_id: Id<GuildMarker>,
        module: impl Into<String>,
        scope: impl Into<String>,
    ) -> Self {
        Self {
            guild_id,
            module: module.into(),
            scope: scope.into(),
        }
    }

    pub async fn get<T>(&self, db: &PgPool) -> Result<Option<GuildSettings<T>>, Error>
    where
        T: Unpin + Sync + Send + Serialize + for<'de> Deserialize<'de>,
    {
        get_guild_settings(db, self.guild_id, &self.module, &self.scope).await
    }
}

pub async fn get_guild_settings<T>(
    db: &PgPool,
    guild_id: Id<GuildMarker>,
    module: &str,
    scope: &str,
) -> Result<Option<GuildSettings<T>>, Error>
where
    T: Unpin + Send + Sync + Serialize + for<'de> Deserialize<'de>,
{
    Ok(sqlx::query_as!(
        GuildSettings,
        r#"
            SELECT
                id,
                guild_id,
                module,
                scope,
                data AS "data: Json<T>",
                created_at,
                updated_at
            FROM
                guild_settings
            WHERE
                guild_id = $1
                AND module = $2
                AND scope = $3
        "#,
        i64::from(DbId(guild_id)),
        module,
        scope
    )
    .fetch_optional(db)
    .await?)
}
