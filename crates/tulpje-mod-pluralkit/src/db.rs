use pkrs_fork::model::System;
use sqlx::prelude::FromRow;
use twilight_model::id::{
    Id,
    marker::{GuildMarker, UserMarker},
};

use tulpje_framework::Error;
use uuid::Uuid;

use super::util::SystemRef;
use tulpje_lib::db::DbId;

#[derive(Debug)]
pub(crate) struct ModPkGuildRow {
    pub(crate) guild_id: DbId<GuildMarker>,
    pub(crate) user_id: DbId<UserMarker>,
    pub(crate) system_uuid: Uuid,
}
pub(crate) async fn save_guild_settings(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
    system_uuid: Uuid,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO pk_guilds (guild_id, user_id, system_uuid) VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO UPDATE SET system_uuid = $3",
        i64::from(DbId(guild_id)),
        i64::from(DbId(user_id)),
        system_uuid,
    )
    .execute(db)
    .await?;

    Ok(())
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn get_guild_settings_for_system(
    db: &sqlx::PgPool,
    system_uuid: Uuid,
) -> Result<Option<ModPkGuildRow>, Error> {
    Ok(sqlx::query_as!(
        ModPkGuildRow,
        "SELECT guild_id, user_id, system_uuid FROM pk_guilds WHERE system_uuid = $1",
        system_uuid
    )
    .fetch_optional(db)
    .await?)
}

pub(crate) async fn get_guild_settings_for_id(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<Option<ModPkGuildRow>, Error> {
    Ok(sqlx::query_as!(
        ModPkGuildRow,
        "SELECT guild_id, user_id, system_uuid FROM pk_guilds WHERE guild_id = $1",
        i64::from(DbId(guild_id))
    )
    .fetch_optional(db)
    .await?)
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn get_guild_settings(db: &sqlx::PgPool) -> Result<Vec<ModPkGuildRow>, Error> {
    Ok(sqlx::query_as!(
        ModPkGuildRow,
        "SELECT guild_id, user_id, system_uuid FROM pk_guilds",
    )
    .fetch_all(db)
    .await?)
}

#[derive(Debug, FromRow)]
#[expect(dead_code, reason = "reflects database structure")]
pub(crate) struct ModPkSystem {
    pub(crate) id: String,
    pub(crate) uuid: Uuid,
    pub(crate) name: Option<String>,
    pub(crate) avatar: Option<String>,
    pub(crate) created_at: chrono::NaiveDateTime,
    pub(crate) updated_at: chrono::NaiveDateTime,
}

impl From<System> for ModPkSystem {
    fn from(value: System) -> Self {
        Self {
            id: value.id.0,
            uuid: value.uuid,
            name: value.name,
            avatar: value.avatar_url.map(|url| url.to_string()),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn get_all_systems(db: &sqlx::PgPool) -> Result<Vec<ModPkSystem>, Error> {
    Ok(sqlx::query_as!(
        ModPkSystem,
        r#"
            SELECT
                id,
                uuid,
                name,
                avatar,
                created_at,
                updated_at
            FROM
                pk_systems
        "#,
    )
    .fetch_all(db)
    .await?)
}
pub(crate) async fn get_systems(
    db: &sqlx::PgPool,
    uuids: Vec<Uuid>,
) -> Result<Vec<ModPkSystem>, Error> {
    Ok(sqlx::query_as!(
        ModPkSystem,
        r#"
            SELECT
                id,
                uuid,
                name,
                avatar,
                created_at,
                updated_at
            FROM
                pk_systems
            WHERE
                uuid = ANY($1)
        "#,
        &uuids[..],
    )
    .fetch_all(db)
    .await?)
}

pub(crate) async fn get_system(
    db: &sqlx::PgPool,
    system_ref: &SystemRef,
) -> Result<Option<ModPkSystem>, Error> {
    match system_ref {
        SystemRef::Id(id) => Ok(sqlx::query_as!(
            ModPkSystem,
            r#"
                SELECT
                    id,
                    uuid,
                    name,
                    avatar,
                    created_at,
                    updated_at
                FROM
                    pk_systems
                WHERE
                    id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?),
        SystemRef::Uuid(uuid) => Ok(sqlx::query_as!(
            ModPkSystem,
            r#"
                SELECT
                    id,
                    uuid,
                    name,
                    avatar,
                    created_at,
                    updated_at
                FROM
                    pk_systems
                WHERE
                    uuid = $1
            "#,
            uuid
        )
        .fetch_optional(db)
        .await?),
        SystemRef::DiscordId(_) => Err("Deleting by discord ID is unsupported".into()),
    }
}

pub(crate) async fn get_system_for_guild(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<Option<ModPkSystem>, Error> {
    Ok(sqlx::query_as!(
        ModPkSystem,
        r#"
            SELECT
                pk_systems.id,
                pk_systems.uuid,
                pk_systems.name,
                pk_systems.avatar,
                pk_systems.created_at,
                pk_systems.updated_at
            FROM
                pk_guilds
            INNER JOIN
                pk_systems
            ON
                pk_systems.uuid = pk_guilds.system_uuid
            WHERE
                guild_id = $1
        "#,
        i64::from(DbId(guild_id))
    )
    .fetch_optional(db)
    .await?)
}

pub(crate) async fn update_system(db: &sqlx::PgPool, system: &ModPkSystem) -> Result<(), Error> {
    sqlx::query!(
        r#"
            INSERT INTO
                pk_systems (id, uuid, name, avatar)
            VALUES
                ($1, $2, $3, $4)
            ON CONFLICT
                (uuid)
            DO UPDATE SET
                id = $1,
                name = $3,
                avatar = $4,
                updated_at = NOW()
        "#,
        system.id,
        system.uuid,
        system.name,
        system.avatar,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn touch_system(db: &sqlx::PgPool, uuid: Uuid) -> Result<(), Error> {
    sqlx::query!(
        r#"
            UPDATE
                pk_systems
            SET
                updated_at = NOW()
            WHERE
                uuid = $1
        "#,
        uuid
    )
    .execute(db)
    .await
    .map_err(|err| format!("error updating `pk_systems.updated_at` for system {uuid}: {err}"))?;

    Ok(())
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn delete_system(db: &sqlx::PgPool, system_ref: SystemRef) -> Result<(), Error> {
    match system_ref {
        SystemRef::Uuid(uuid) => {
            sqlx::query!("DELETE FROM pk_systems WHERE uuid = $1", uuid)
                .execute(db)
                .await?;
            Ok(())
        }
        SystemRef::Id(id) => {
            sqlx::query!("DELETE FROM pk_systems WHERE id = $1", id,)
                .execute(db)
                .await?;
            Ok(())
        }
        SystemRef::DiscordId(_) => Err("Deleting by discord ID is unsupported".into()),
    }
}

pub(crate) async fn cleanup_systems(db: &sqlx::PgPool) -> Result<u64, Error> {
    Ok(sqlx::query!(
        r#"
            DELETE
            FROM
                pk_systems
            WHERE
                NOT EXISTS (
                    SELECT 1 FROM pk_guilds WHERE pk_guilds.system_uuid = pk_systems.uuid
                )
            AND
                NOT EXISTS (
                    SELECT 1 FROM pk_notify_systems WHERE pk_notify_systems.system_uuid = pk_systems.uuid
                )
        "#
    )
    .execute(db)
    .await?
    .rows_affected())
}

/// get a list of systems that haven't been updated in 24 hours
pub(crate) async fn get_systems_to_update(db: &sqlx::PgPool) -> Result<Vec<ModPkSystem>, Error> {
    Ok(sqlx::query_as!(
        ModPkSystem,
        r#"
            SELECT
                pk_systems.uuid,
                pk_systems.id,
                pk_systems.name,
                pk_systems.avatar,
                pk_systems.created_at,
                pk_systems.updated_at
            FROM
                pk_systems
            WHERE
                pk_systems.updated_at <= NOW() - interval '24 hours'
            AND
                uuid IN (SELECT uuid FROM pk_tracked_systems)
            ORDER BY
                updated_at
            ASC
            LIMIT 5
        "#
    )
    .fetch_all(db)
    .await?)
}

pub(crate) async fn get_outdated_system_count(db: &sqlx::PgPool) -> Result<usize, Error> {
    Ok(sqlx::query_scalar!(
        r#"
            SELECT
                COUNT(uuid) AS "count!"
            FROM
                pk_systems
            WHERE
                updated_at <= NOW() - interval '24 hours'
            AND
                uuid IN (SELECT uuid FROM pk_tracked_systems)
        "#
    )
    .fetch_one(db)
    .await? as usize)
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::test_utils::create_n_systems;

    use super::*;

    #[ignore]
    #[sqlx::test(migrations = "../../migrations")]
    async fn test_get_outdated_system_count_up_to_date(
        db: sqlx::PgPool,
    ) -> Result<(), tulpje_framework::Error> {
        let updated_at = chrono::Utc::now().naive_utc();
        create_n_systems(&db, 50, updated_at).await?;
        assert_eq!(get_outdated_system_count(&db).await?, 0);
        Ok(())
    }

    #[ignore]
    #[sqlx::test(migrations = "../../migrations")]
    async fn test_get_outdated_system_count_outdated(
        db: sqlx::PgPool,
    ) -> Result<(), tulpje_framework::Error> {
        let updated_at = chrono::Utc::now().naive_utc() - chrono::Duration::hours(48);
        create_n_systems(&db, 50, updated_at).await?;
        assert_eq!(get_outdated_system_count(&db).await?, 50);
        Ok(())
    }

    #[ignore]
    #[sqlx::test(migrations = "../../migrations")]
    async fn test_get_outdated_system_count_split(
        db: sqlx::PgPool,
    ) -> Result<(), tulpje_framework::Error> {
        let updated_at = chrono::Utc::now().naive_utc() - chrono::Duration::hours(48);
        let now = chrono::Utc::now().naive_utc();
        create_n_systems(&db, 25, now).await?;
        create_n_systems(&db, 25, updated_at).await?;
        assert_eq!(get_outdated_system_count(&db).await?, 25);
        Ok(())
    }
}
