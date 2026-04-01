use chrono::NaiveDateTime;
use tulpje_framework::Error;
use tulpje_lib::db_id::DbId;
use twilight_model::id::{
    Id,
    marker::{GuildMarker, RoleMarker},
};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub(super) struct ModPkGuildRole {
    pub(super) id: i32,

    pub(super) guild_id: DbId<GuildMarker>,
    pub(super) role_id: DbId<RoleMarker>,
    pub(super) member_uuid: Uuid,

    pub(super) created_at: NaiveDateTime,
    pub(super) updated_at: NaiveDateTime,
}

pub(super) async fn get_guild_roles(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<Vec<ModPkGuildRole>, Error> {
    Ok(sqlx::query_as!(
        ModPkGuildRole,
        r#"
            SELECT
                id,

                guild_id,
                role_id,
                member_uuid,

                created_at,
                updated_at
            FROM
                pk_guild_roles
            WHERE
                guild_id = $1;
        "#,
        i64::from(DbId(guild_id))
    )
    .fetch_all(db)
    .await
    .map_err(|err| format!("error fetching results for get_guild_roles: {err}"))?)
}

pub(super) async fn count_guild_roles(
    sqlx: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<usize, Error> {
    Ok(sqlx::query_scalar!(
        r#"
            SELECT
                COUNT(id) AS "count!"
            FROM
                pk_guild_roles
            WHERE
                guild_id = $1;
        "#,
        i64::from(DbId(guild_id))
    )
    .fetch_one(sqlx)
    .await
    .map_err(|err| format!("error fetching results for count_guild_roles: {err}"))? as usize)
}
