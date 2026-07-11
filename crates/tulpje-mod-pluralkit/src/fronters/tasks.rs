use std::{slice, sync::Arc};

use pkrs_fork::{client::PkClient, model::Member};
use tracing::instrument;
use tulpje_lib::util::{ERROR_UNKNOWN_CHANNEL, get_json_error_code, warning_message};
use twilight_http::Client;
use twilight_model::{
    channel::message::{Component, MessageFlags, component::TextDisplay},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
};

use tulpje_framework::Error;
use twilight_util::builder::message::{ContainerBuilder, SeparatorBuilder};

use crate::{
    db::ModPkSystem,
    fronters::{
        db::{self, delete_fronter_category},
        shared::{FrontChange, GetSystemFrontersError, Switch, update_system_fronters},
    },
    notify::db::{
        self as notify_db, delete_notify_channel, delete_notify_systems, get_notify_channel,
    },
    util::get_member_name,
};

// type of notification we're sending to the guild
enum GuildNotification {
    Switch(Component),
    NotFound(Component),
    Private(Component),
}

impl GuildNotification {
    /// return the inner component of the notification
    fn component(&self) -> &Component {
        match self {
            Self::Switch(component) | Self::NotFound(component) | Self::Private(component) => {
                component
            }
        }
    }

    /// description to use in logging
    fn description(&self) -> &'static str {
        match self {
            Self::Switch(_) => "switch",
            Self::Private(_) => "front is private",
            Self::NotFound(_) => "system not found",
        }
    }

    /// type field of the metric for this notification type
    fn metric_type(&self) -> &'static str {
        match self {
            Self::Switch(_) => "success",
            Self::NotFound(_) => "notfound",
            Self::Private(_) => "private",
        }
    }
}

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
                method = "update_fronter_categories",
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
fn create_front_change_component(
    system: &ModPkSystem,
    switch: &Switch,
) -> Result<Component, Error> {
    let mut embed_lines = vec![format!(
        "### Switch: {}",
        system.name.as_ref().unwrap_or(&system.id)
    )];
    for member in switch.fronters.iter().take(MAX_FRONTERS_IN_MESSAGE) {
        embed_lines.push(format!("* {}", get_member_name(member)));
    }

    if switch.fronters.len() > MAX_FRONTERS_IN_MESSAGE {
        embed_lines.push(format!(
            "-# and {} more",
            switch.fronters.len() - MAX_FRONTERS_IN_MESSAGE
        ));
    }

    let unix_time_secs = switch.timestamp.and_utc().timestamp();

    Ok(ContainerBuilder::new()
        .component(TextDisplay {
            id: None,
            content: embed_lines.join("\n"),
        })
        .component(SeparatorBuilder::new().divider(false).build())
        .component(TextDisplay {
            id: None,
            content: format!("-# <t:{unix_time_secs}:f>"),
        })
        .build()
        .into())
}

#[tracing::instrument(skip_all)]
async fn notify_guilds_for_system(
    db: &sqlx::PgPool,
    discord_client: &Arc<Client>,
    system: &ModPkSystem,
    notification: &GuildNotification,
) -> Result<Vec<Id<GuildMarker>>, Error> {
    let guilds = notify_db::get_notify_guilds_for_system(db, system.uuid).await?;

    tracing::debug!(
        method = "notify_front_change",
        "notifying {} guilds about system {}: {}",
        guilds.len(),
        system.uuid,
        notification.description()
    );

    let mut guilds_successfully_notified = Vec::new();
    for guild_id in guilds {
        tracing::debug!(
            "notifying guild {} for system {}: {}",
            guild_id,
            system.uuid,
            notification.description()
        );

        if let Err(err) = notify_guild(db, discord_client, guild_id, notification).await {
            tracing::warn!(
                "error notifying guild {} for system {}: {}",
                guild_id,
                system.uuid,
                err
            );
            continue;
        };

        guilds_successfully_notified.push(guild_id);
    }

    Ok(guilds_successfully_notified)
}

#[tracing::instrument(skip_all)]
async fn notify_guild(
    db: &sqlx::PgPool,
    discord_client: &Arc<Client>,
    guild_id: Id<GuildMarker>,
    notification: &GuildNotification,
) -> Result<(), Error> {
    metrics::counter!("pk:notifications", "type" => "total").increment(1);

    let Some(channel_id) = get_notify_channel(db, guild_id).await? else {
        metrics::counter!("pk:notifications", "type" => "channel-missing").increment(1);
        return Err(format!(
            "no notify channel configured for guild {guild_id} despite it having tracked systems"
        )
        .into());
    };

    match discord_client
        .create_message(*channel_id)
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(slice::from_ref(notification.component()))
        .await
    {
        Err(err) if get_json_error_code(&err).is_some_and(|code| code == ERROR_UNKNOWN_CHANNEL) => {
            // channel was deleted remove it from pk_notify_channels
            tracing::info!(
                "received ERROR_UNKNOWN_CHANNEL for category {channel_id} \
                in guild {guild_id}, removing from notify channel config"
            );
            delete_notify_channel(db, *channel_id)
                .await
                .map_err(|err| {
                    format!(
                        "error deleting notify channel {channel_id} for guild {guild_id}: {err}"
                    )
                })?;
            delete_notify_systems(db, guild_id).await.map_err(|err| {
                format!("error deleting notify channel {channel_id} for guild {guild_id}: {err}")
            })?;

            Ok(())
        }
        Err(err) => {
            metrics::counter!("pk:notifications", "type" => "error").increment(1);
            return Err(format!(
                "error sending notification to guild {guild_id} channel {channel_id}: {err}",
            )
            .into());
        }
        Ok(_) => {
            metrics::counter!("pk:notifications", "type" => notification.metric_type())
                .increment(1);
            Ok(())
        }
    }
}

async fn notify_system_not_found(
    db: &sqlx::PgPool,
    discord_client: &Arc<Client>,
    system: &ModPkSystem,
) -> Result<(), Error> {
    let notification = GuildNotification::NotFound(warning_message(&format!(
        "### System Unfollowed\nSystem `{}` has been deleted from PluralKit, and has been unfollowed",
        system.name.as_ref().unwrap_or(&system.id)
    )));

    let guilds_successfully_notified =
        notify_guilds_for_system(db, discord_client, system, &notification).await?;

    // remove system from guilds we succesfully notified
    notify_db::remove_notify_system_from_guilds(db, system.uuid, guilds_successfully_notified)
        .await?;

    Ok(())
}
async fn notify_front_private(
    db: &sqlx::PgPool,
    discord_client: &Arc<Client>,
    system: &ModPkSystem,
) -> Result<(), Error> {
    let notification = GuildNotification::Private(warning_message(&format!(
        "### System Unfollowed\nCurrent fronters for `{}` are private, system unfollowed",
        system.name.as_ref().unwrap_or(&system.id)
    )));

    let guilds_successfully_notified =
        notify_guilds_for_system(db, discord_client, system, &notification).await?;

    // remove system from guilds we succesfully notified
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
    let notification = GuildNotification::Switch(create_front_change_component(system, switch)?);
    notify_guilds_for_system(db, discord_client, system, &notification).await?;

    Ok(())
}

#[instrument("process-system", skip_all, fields(system=?system.uuid))]
pub(crate) async fn process_system(
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
        Err(GetSystemFrontersError::NotFound(_)) => {
            notify_system_not_found(db, discord_client, system).await?;
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
