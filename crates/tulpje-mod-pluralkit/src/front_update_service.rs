use std::time::Duration;

use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;
use tulpje_framework::Error;
use tulpje_lib::context::TaskContext;

use crate::fronters::{db, tasks::process_system};

async fn tick(ctx: &TaskContext) -> Result<(), Error> {
    let tracked_system_count = db::get_tracked_system_count(&ctx.services.db).await?;
    metrics::counter!("pk:tracked-systems").absolute(tracked_system_count as u64);

    let system_count = db::get_system_count(&ctx.services.db).await?;
    metrics::counter!("pk:total-systems").absolute(system_count as u64);

    let outdated_fronters = db::get_outdated_fronter_count(&ctx.services.db).await?;
    metrics::counter!("pk:outdated-fronters").absolute(outdated_fronters as u64);

    let systems_to_update = db::get_systems_to_update(&ctx.services.db).await?;

    for system in &systems_to_update {
        if let Err(err) =
            process_system(&ctx.services.db, &ctx.services.pk, &ctx.client, system).await
        {
            tracing::warn!("error updating system {}: {}", system.uuid, err);
        }
    }

    Ok(())
}

pub(crate) async fn start(ctx: TaskContext, shutdown: CancellationToken) -> Result<(), Error> {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
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
