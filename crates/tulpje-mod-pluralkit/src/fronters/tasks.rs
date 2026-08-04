use std::{slice, sync::Arc};

use pkrs_fork::{client::PkClient, model::Member};
use tracing::instrument;
use tulpje_lib::util::{ERROR_UNKNOWN_CHANNEL, get_json_error_code, warning_message};
use twilight_http::Client;
use twilight_model::{
    channel::message::{
        Component, Embed, MessageFlags,
        component::{Section, TextDisplay, Thumbnail, UnfurledMediaItem},
    },
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
    util::Timestamp,
};

use tulpje_framework::Error;
use twilight_util::builder::{
    embed::{EmbedBuilder, ImageSource},
    message::{ContainerBuilder, SeparatorBuilder},
};

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
    Switch(Box<Embed>),
    NotFound(Component),
    Private(Component),
}

impl GuildNotification {
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
#[expect(
    dead_code,
    reason = "want to keep the reference components v2 implementation around"
)]
fn create_front_change_component(
    system: &ModPkSystem,
    switch: &Switch,
) -> Result<Component, Error> {
    let system_name = system.name.as_ref().unwrap_or(&system.id);
    let mut embed_lines = vec![format!("### Switch: {system_name}",)];

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

    let front_list = TextDisplay {
        id: None,
        content: embed_lines.join("\n"),
    };

    // handle system avatar
    let front_component: Component = match &system.avatar {
        Some(url) => Section {
            id: None,
            components: vec![front_list.into()],
            accessory: Box::new(
                Thumbnail {
                    id: None,
                    description: Some(Some(format!("avatar for {system_name}"))),
                    media: UnfurledMediaItem {
                        url: url.clone(),
                        proxy_url: None,
                        height: None,
                        width: None,
                        content_type: None,
                    },
                    spoiler: None,
                }
                .into(),
            ),
        }
        .into(),
        None => front_list.into(),
    };

    Ok(ContainerBuilder::new()
        .component(front_component)
        .component(SeparatorBuilder::new().divider(false).build())
        .component(TextDisplay {
            id: None,
            content: format!("-# <t:{unix_time_secs}:f>"),
        })
        .build()
        .into())
}

// NOTE: We're using this because new components don't show in
//       mobile notifications and embeds do
fn create_front_change_embed(system: &ModPkSystem, switch: &Switch) -> Result<Embed, Error> {
    let mut builder = EmbedBuilder::new().title(format!(
        "Switch: {}",
        system.name.as_ref().unwrap_or(&system.id)
    ));

    if let Some(url) = &system.avatar {
        match ImageSource::url(url) {
            Ok(image_source) => builder = builder.thumbnail(image_source),
            Err(err) => tracing::warn!("error parsing system avatar as url: {err}"),
        };
    }

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

    let mut message = discord_client.create_message(*channel_id);
    match notification {
        GuildNotification::NotFound(component) | GuildNotification::Private(component) => {
            message = message
                .flags(MessageFlags::IS_COMPONENTS_V2)
                .components(slice::from_ref(component));
        }
        GuildNotification::Switch(embed) => message = message.embeds(slice::from_ref(embed)),
    }

    match message.await {
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
    let notification = GuildNotification::Switch(create_front_change_embed(system, switch)?.into());
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
