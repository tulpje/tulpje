use tulpje_framework::Error;
use twilight_model::id::{Id, marker::GuildMarker};

use crate::db::DbId;

/// track that we've seen the guild in the database
pub async fn touch(db: &sqlx::PgPool, guild_id: Id<GuildMarker>) -> Result<(), Error> {
    sqlx::query!(
        r#"
            INSERT INTO
                guilds (guild_id, created_at, updated_at)
            VALUES
                ($1, NOW(), NOW())
            ON CONFLICT
                (guild_id)
            DO UPDATE SET
                updated_at = NOW(),
                deleted_at = NULL
        "#,
        i64::from(DbId(guild_id))
    )
    .execute(db)
    .await?;

    Ok(())
}
