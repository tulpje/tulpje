use std::time::Duration;

use pluralkit_rs::{PluralKit, models::PluralKitError};
use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;
use tulpje_framework::Error;
use tulpje_lib::context::TaskContext;

use crate::db::{self, ModPkSystem};

async fn process_system(
    db: &sqlx::PgPool,
    pk: &PluralKit,
    system: &ModPkSystem,
) -> Result<(), Error> {
    match pk.get_system(&system.uuid.into()).await {
        Ok(response) => {
            db::update_system(db, &response.model().await?.into()).await?;
        }
        // 20001 = system not found
        Err(PluralKitError::PluralKit { code: 20001, .. }) => {
            db::touch_system(db, system.uuid).await?;
        }
        Err(err) => return Err(err.into()),
    };

    Ok(())
}

async fn tick(ctx: &TaskContext) -> Result<(), Error> {
    let systems_to_update = db::get_systems_to_update(&ctx.services.db).await?;

    let outdated_systems = db::get_outdated_system_count(&ctx.services.db).await?;
    metrics::counter!("pk:outdated-systems").absolute(outdated_systems as u64);

    for system in &systems_to_update {
        tracing::info!("updating system {}", system.uuid);
        if let Err(err) = process_system(&ctx.services.db, &ctx.services.pk, system).await {
            tracing::warn!("error updating system {}: {}", system.uuid, err);
        }
    }

    Ok(())
}

pub(crate) async fn start(ctx: TaskContext, shutdown: CancellationToken) -> Result<(), Error> {
    let mut interval = tokio::time::interval(Duration::from_mins(10));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(err) = tick(&ctx).await {
                    tracing::warn!("error updating fronters: {err}");
                }
            }
            () = shutdown.cancelled() => break,
        }
    }

    Ok(())
}
