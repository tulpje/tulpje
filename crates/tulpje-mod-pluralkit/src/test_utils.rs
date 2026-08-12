use tulpje_lib::db::guild::touch;
use twilight_model::id::{Id, marker::GuildMarker};
use uuid::Uuid;

use crate::{
    db::{ModPkSystem, update_system},
    fronters::db::get_system_count,
    notify::db::add_notify_system,
};

pub(crate) async fn create_n_systems(
    db: &sqlx::PgPool,
    n: u16,
    updated_at: chrono::NaiveDateTime,
) -> Result<Vec<Uuid>, tulpje_framework::Error> {
    let guild_id = Id::<GuildMarker>::new(1);
    touch(db, guild_id).await?;

    let now = chrono::Utc::now().naive_utc();
    let offset = get_system_count(db).await?;
    let mut uuids = Vec::new();
    for i in offset + 1..=offset + (n as usize) {
        let uuid = Uuid::now_v7();
        update_system(
            db,
            &ModPkSystem {
                id: format!("sys{:03}", i),
                uuid,
                name: None,
                avatar: None,
                created_at: now,
                updated_at,
            },
        )
        .await?;
        add_notify_system(db, guild_id, uuid).await?;
        uuids.push(uuid);
    }

    sqlx::query(
        r#"
            UPDATE
                pk_systems
            SET
                updated_at = $1
            WHERE
                uuid = ANY($2)
        "#,
    )
    .bind(updated_at)
    .bind(&uuids)
    .execute(db)
    .await?;

    Ok(uuids)
}

pub(crate) async fn create_n_fronters(
    db: &sqlx::PgPool,
    n: u16,
    updated_at: chrono::NaiveDateTime,
) -> Result<(), tulpje_framework::Error> {
    let guild_id = Id::<GuildMarker>::new(1);
    touch(db, guild_id).await?;

    let system_uuids = create_n_systems(db, n, updated_at).await?;
    for uuid in system_uuids {
        sqlx::query(
            r#"
                    INSERT INTO pk_system_fronters (system_uuid, fronters, updated_at)
                    VALUES
                        ($1, '[]', $2)
                "#,
        )
        .bind(uuid)
        .bind(updated_at)
        .execute(db)
        .await?;
    }
    Ok(())
}
