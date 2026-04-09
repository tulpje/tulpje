use tulpje_framework::Error;
use tulpje_lib::{context::CommandContext, responses};

use super::{db, shared::update_fronter_channels};
use crate::{
    db::{get_guild_settings_for_id, get_system},
    fronters::shared::{GetSystemFrontersError, get_system_fronters},
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?;

    let Some(gs) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        responses::error(&ctx, "PluralKit module not set-up, please run `/pk setup`").await?;
        return Ok(());
    };

    let Some(cat_id) = db::get_fronter_category(&ctx.services.db, guild.id).await? else {
        responses::error(
            &ctx,
            "Fronter category not set-up, please run `/pk fronters setup`",
        )
        .await?;
        return Ok(());
    };

    let cat = ctx.client().channel(*cat_id).await?.model().await?;

    cat.guild_id
        .ok_or_else(|| format!("channel {} isn't a guild channel", cat_id))?;

    let system = get_system(&ctx.services.db, &gs.system_uuid.into())
        .await
        .map_err(|err| format!("error fetching system {}: {}", gs.system_uuid, err))?;
    let display_name =
        system.map_or_else(|| gs.system_uuid.to_string(), |s| s.name.unwrap_or(s.id));

    // TODO: Fix horrible deduplication between this and `update_system_fronters`
    let members = match get_system_fronters(&ctx.services.pk, gs.system_uuid).await {
        Ok(Some(fronters)) => fronters.members,
        Ok(None) => Vec::new(),
        Err(GetSystemFrontersError::Private(_)) => {
            responses::error(
                &ctx,
                &format!(
                    "Fronters for system `{display_name}` are private, \
                    please set them to public to use the fronter category"
                ),
            )
            .await?;

            if let Err(err) = db::update_fronters_timestamp(&ctx.services.db, gs.system_uuid).await
            {
                tracing::warn!(
                    "error updating fronter timestamp in db for system {}: {}",
                    gs.system_uuid,
                    err
                );
            }
            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };

    update_fronter_channels(&ctx.client(), guild, cat, &members).await?;
    let fronter_uuids: Vec<_> = members.iter().map(|m| m.uuid).collect();
    if let Err(err) = db::update_fronters(&ctx.services.db, gs.system_uuid, &fronter_uuids).await {
        tracing::warn!(
            "error updating fronter timestamp in db for system {}: {}",
            gs.system_uuid,
            err
        );
    }

    responses::success(&ctx, "Fronter category updated!").await?;
    Ok(())
}
