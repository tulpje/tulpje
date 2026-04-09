use std::{slice, sync::Arc};

use pkrs_fork::{client::PkClient, model::Member};
use tracing::instrument;
use tulpje_lib::{
    context::TaskContext,
    util::{ERROR_UNKNOWN_CHANNEL, get_json_error_code, warning_message},
};
use twilight_http::Client;
use twilight_model::{
    channel::message::{Embed, MessageFlags},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
    util::Timestamp,
};

use tulpje_framework::Error;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    db::ModPkSystem,
    fronters::{
        db::{self, delete_fronter_category},
        shared::{FrontChange, GetSystemFrontersError, Switch, update_system_fronters},
    },
    notify::db::{self as notify_db, get_notify_channel},
    util::get_member_name,
};

async fn update_fronter_categories(
    db: &sqlx::PgPool,
    discord_client: &Arc<Client>,
    system: &ModPkSystem,
    switch: &Switch,
) -> Result<(), Error> {
    let guild_categories = db::get_fronter_categories_for_system(db, system.uuid)
        .await
        .map_err(|err| {
            format!(
                "error fetching guilds for system {} from db: {}",
                system.uuid, err
            )
        })?;

    tracing::debug!(
        "updating front categories for system {} in {} guilds",
        system.uuid,
        guild_categories.len(),
    );

    for guild_category in guild_categories {
        metrics::counter!("pk:front-category", "type" => "total").increment(1);

        if let Err(err) = update_fronters_for_guild(
            db,
            discord_client,
            *guild_category.guild_id,
            *guild_category.category_id,
            &switch.fronters,
        )
        .await
        {
            metrics::counter!("pk:front-category", "type" => "error").increment(1);
            tracing::error!(
                method = "update_fronter_category",
                "error updating fronters for guild {} category {}: {}",
                guild_category.guild_id,
                guild_category.category_id,
                err
            );
        } else {
            metrics::counter!("pk:front-category", "type" => "success").increment(1);
        }
    }

    Ok(())
}

const MAX_FRONTERS_IN_MESSAGE: usize = 20;
// TODO: Components V2
fn create_front_change_embed(system: &ModPkSystem, switch: &Switch) -> Result<Embed, Error> {
    let builder = EmbedBuilder::new().title(format!(
        "Switch: {}",
        system.name.as_ref().unwrap_or(&system.id)
    ));

    let mut embed_parts = Vec::new();
    for member in switch.fronters.iter().take(MAX_FRONTERS_IN_MESSAGE) {
        embed_parts.push(format!("* {}", get_member_name(member)));
    }

    if switch.fronters.len() > MAX_FRONTERS_IN_MESSAGE {
        embed_parts.push(format!(
            "-# and {} more",
            switch.fronters.len() - MAX_FRONTERS_IN_MESSAGE
        ));
    }

    Ok(builder
        .description(embed_parts.join("\n"))
        .timestamp(Timestamp::from_secs(
            switch.timestamp.and_utc().timestamp(),
        )?)
        .validate()?
        .build())
}

async fn notify_front_private(
    db: &sqlx::PgPool,
    discord_client: &Arc<Client>,
    system: &ModPkSystem,
) -> Result<(), Error> {
    let guilds = notify_db::get_notify_guilds_for_system(db, system.uuid).await?;
    tracing::debug!(
        method = "notify_front_change",
        "notifying {} guilds of front for {} being private",
        guilds.len(),
        system.uuid
    );

    let message = warning_message(&format!(
        "### System Unfollowed\nCurrent fronters for `{}` are private, system unfollowed",
        system.name.as_ref().unwrap_or(&system.id)
    ));

    let mut guilds_successfully_notified = Vec::new();
    for guild_id in guilds {
        metrics::counter!("pk:notifications", "type" => "total").increment(1);
        let Some(channel_id) = get_notify_channel(db, guild_id).await? else {
            metrics::counter!("pk:notifications", "type" => "channel-missing").increment(1);
            tracing::warn!(
                method = "notify_front_private",
                "no notify channel configured for guild {} despite it having tracked systems",
                guild_id,
            );
            continue;
        };

        if let Err(err) = discord_client
            .create_message(*channel_id)
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(slice::from_ref(&message))
            .await
        {
            metrics::counter!("pk:notifications", "type" => "error").increment(1);
            tracing::warn!(
                method = "notify_front_private",
                "error sending front private notification to guild {} channel {}: {}",
                guild_id,
                channel_id,
                err
            );
        } else {
            metrics::counter!("pk:notifications", "type" => "private").increment(1);
            guilds_successfully_notified.push(guild_id);
        }
    }

    notify_db::remove_notify_system_from_guilds(db, system.uuid, guilds_successfully_notified)
        .await?;

    Ok(())
}

async fn notify_front_change(
    db: &sqlx::PgPool,
    discord_client: &Arc<Client>,
    system: &ModPkSystem,
    switch: &Switch,
) -> Result<(), Error> {
    let embed = create_front_change_embed(system, switch)?;

    let guilds = notify_db::get_notify_guilds_for_system(db, system.uuid).await?;
    tracing::debug!(
        method = "notify_front_change",
        "notifying {} guilds of front change in {}",
        guilds.len(),
        system.id
    );
    for guild_id in guilds {
        metrics::counter!("pk:notifications", "type" => "total").increment(1);
        tracing::debug!(
            method = "notify_front_change",
            "notifying guild {} of front change in {}",
            guild_id,
            system.id
        );

        let Some(channel_id) = get_notify_channel(db, guild_id).await? else {
            metrics::counter!("pk:notifications", "type" => "channel-missing").increment(1);
            tracing::warn!(
                method = "notify_front_change",
                "no notify channel configured for guild {} despite it having tracked systems",
                guild_id,
            );
            continue;
        };

        if let Err(err) = discord_client
            .create_message(*channel_id)
            .embeds(slice::from_ref(&embed))
            .await
        {
            metrics::counter!("pk:notifications", "type" => "error").increment(1);
            tracing::warn!(
                method = "notify_front_change",
                "error sending front change notification to guild {} channel {}: {}",
                guild_id,
                channel_id,
                err
            );
        } else {
            metrics::counter!("pk:notifications", "type" => "success").increment(1);
        }
    }
    Ok(())
}

#[instrument("process-system", skip_all, fields(system=?system.uuid))]
async fn process_system(
    db: &sqlx::PgPool,
    pk_client: &PkClient,
    discord_client: &Arc<Client>,
    system: &ModPkSystem,
) -> Result<(), Error> {
    let changed = match update_system_fronters(db, system, pk_client).await {
        Ok(changed) => changed,
        Err(GetSystemFrontersError::Private(_)) => {
            notify_front_private(db, discord_client, system).await?;
            return Ok(());
        }
        Err(err) => return Err(format!("error updating system fronters: {err}").into()),
    };
    match changed {
        FrontChange::Changed(switch) => {
            tracing::debug!("fronters changed for system {}", system.uuid);
            update_fronter_categories(db, discord_client, system, &switch).await?;
            notify_front_change(db, discord_client, system, &switch).await?;
        }
        FrontChange::Unchanged => {
            tracing::debug!("fronters unchanged for system {}", system.uuid);
        }
    }
    Ok(())
}

pub(crate) async fn update_fronters(ctx: TaskContext) -> Result<(), Error> {
    let tracked_system_count = db::get_tracked_system_count(&ctx.services.db).await?;
    metrics::counter!("pk:tracked-systems").absolute(tracked_system_count as u64);

    let system_count = db::get_system_count(&ctx.services.db).await?;
    metrics::counter!("pk:total-systems").absolute(system_count as u64);

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

async fn update_fronters_for_guild(
    db: &sqlx::PgPool,
    client: &Client,
    guild_id: Id<GuildMarker>,
    category_id: Id<ChannelMarker>,
    members: &[Member],
) -> Result<(), Error> {
    let guild = client.guild(guild_id).await?.model().await?;

    let category = match client.channel(category_id).await {
        Ok(response) => response.model().await?,
        Err(err) if get_json_error_code(&err).is_some_and(|code| code == ERROR_UNKNOWN_CHANNEL) => {
            // channel was deleted removed it from fronter_categories
            tracing::info!(
                "received ERROR_UNKNOWN_CHANNEL for category {category_id} \
                in guild {guild_id}, removing from fronter category config"
            );
            delete_fronter_category(db, category_id)
                .await
                .map_err(|err| {
                    format!(
                        "error deleting fronter category {category_id} for guild {guild_id}: {err}"
                    )
                })?;
            return Ok(());
        }
        Err(err) => {
            return Err(format!(
                "error fetching fronter category {category_id} for guild {guild_id}: {err}",
            )
            .into());
        }
    };

    category.guild_id.ok_or_else(|| {
        format!(
            "caetgory {} for guild {} isn't a guild channel",
            category.id, guild.id
        )
    })?;

    super::shared::update_fronter_channels(client, &guild, &category, members)
        .await
        .map_err(|err| format!("error updating fronters for guild {}: {}", guild.id, err))?;

    tracing::info!("fronters updated in guild {}", guild.id);
    Ok(())
}
