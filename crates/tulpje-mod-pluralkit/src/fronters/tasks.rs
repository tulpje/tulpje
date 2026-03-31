use std::{slice, sync::Arc};

use chrono::{DateTime, NaiveDateTime, Utc};
use pkrs_fork::{
    client::{PkClient, PluralKitError},
    model::{Member, PkId, Switch as PkSwitch},
};
use serde_either::StringOrStruct;
use tulpje_cache::Cache;
use tulpje_lib::{context::TaskContext, util::warning_message};
use twilight_http::Client;
use twilight_model::{
    channel::message::{Embed, MessageFlags},
    id::{Id, marker::ChannelMarker},
    util::Timestamp,
};

use tulpje_framework::Error;
use twilight_util::builder::embed::EmbedBuilder;
use uuid::Uuid;

use crate::{
    db::{self as pk_db, ModPkGuildRow, ModPkSystem},
    fronters::db,
    notify::db::{self as notify_db, get_notify_channel},
    util::get_member_name,
};

enum FrontChange {
    Unchanged,
    Changed(Switch),
}

struct Switch {
    pub(crate) fronters: Vec<Member>,
    pub(crate) timestamp: NaiveDateTime,
}

#[derive(thiserror::Error, Debug)]
enum UpdateSystemFrontersError {
    #[error("fronters for system {0} are private")]
    Private(Uuid),
    #[error(transparent)]
    Other(#[from] Error),
}

async fn update_system_fronters(
    db: &sqlx::PgPool,
    system: &ModPkSystem,
    client: &PkClient,
) -> Result<FrontChange, UpdateSystemFrontersError> {
    let latest_switch = match client
        .get_system_fronters(&PkId(system.uuid.to_string()))
        .await
    {
        Ok(front) => Ok::<_, UpdateSystemFrontersError>(front),
        // handle private fronters
        Err(PluralKitError::Pk(_, error))
            // 30004 = private fronters
            if error.code == 30004 =>
        {
            db::update_fronters_timestamp(db, system.uuid).await?;
            Err(UpdateSystemFrontersError::Private(system.uuid))
        }
        // directly return any other errors
        Err(err) => Err(UpdateSystemFrontersError::Other(err.into())),
    }?;

    let timestamp = if let Some(ref switch) = latest_switch {
        DateTime::from_timestamp(switch.timestamp.to_utc().unix_timestamp(), 0)
            .ok_or_else(|| {
                UpdateSystemFrontersError::Other(
                    format!(
                        "timestamp out of range: {}",
                        switch.timestamp.to_utc().unix_timestamp()
                    )
                    .into(),
                )
            })?
            .naive_utc()
    } else {
        Utc::now().naive_utc()
    };
    let fronters = gather_fronters_from_switch(system.uuid, latest_switch)?;
    let fronter_uuids: Vec<_> = fronters.iter().map(|f| f.uuid).collect();

    if db::did_fronters_change(db, system.uuid, &fronter_uuids).await? {
        // update the fronters in the db if they changed
        db::update_fronters(db, system.uuid, &fronter_uuids).await?;
        Ok(FrontChange::Changed(Switch {
            fronters,
            timestamp,
        }))
    } else {
        // otherwise just update the `updated_at` timestamp
        db::update_fronters_timestamp(db, system.uuid).await?;
        Ok(FrontChange::Unchanged)
    }
}

fn gather_fronters_from_switch(
    system_uuid: Uuid,
    switch: Option<PkSwitch>,
) -> Result<Vec<Member>, Error> {
    let Some(switch) = switch else {
        return Ok(Vec::new());
    };

    let mut fronters = Vec::<Member>::new();
    for member in switch.members {
        match member {
            StringOrStruct::String(_) => Err(format!(
                "system {system_uuid} returned uuids instead of member structs",
            ))?,
            StringOrStruct::Struct(member) => fronters.push(member),
        };
    }

    Ok(fronters)
}

async fn update_fronter_category(
    db: &sqlx::PgPool,
    pk: &PkClient,
    discord_client: &Arc<Client>,
    cache: &Cache,
    system: &ModPkSystem,
    switch: &Switch,
) -> Result<(), Error> {
    let Some(guild_settings) = pk_db::get_guild_settings_for_system(db, system.uuid).await? else {
        tracing::debug!(
            method = "update_fronter_category",
            "no guild with system {}, skipping",
            system.id
        );
        return Ok(());
    };

    metrics::counter!("pk:front-category", "type" => "total").increment(1);
    let Some(category_id) = db::get_fronter_category(db, *guild_settings.guild_id).await? else {
        metrics::counter!("pk:front-category", "type" => "category-missing").increment(1);
        tracing::debug!(
            method = "update_fronter_category",
            "no fronter category configured for guild {}, skipping",
            guild_settings.guild_id
        );
        return Ok(());
    };

    if let Err(err) = update_fronters_for_guild(
        discord_client,
        pk,
        cache,
        &guild_settings,
        *category_id,
        &switch.fronters,
    )
    .await
    {
        metrics::counter!("pk:front-category", "type" => "error").increment(1);
        tracing::error!(
            method = "update_fronter_category",
            "error updating fronters for guild {} category {}: {}",
            guild_settings.guild_id,
            category_id,
            err
        );
    } else {
        metrics::counter!("pk:front-category", "type" => "success").increment(1);
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

async fn process_system(
    db: &sqlx::PgPool,
    pk_client: &PkClient,
    discord_client: &Arc<Client>,
    cache: &Cache,
    system: &ModPkSystem,
) -> Result<(), Error> {
    let changed = match update_system_fronters(db, system, pk_client).await {
        Ok(changed) => changed,
        Err(UpdateSystemFrontersError::Private(_)) => {
            notify_front_private(db, discord_client, system).await?;
            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };
    match changed {
        FrontChange::Changed(switch) => {
            tracing::debug!("fronters changed for system {}", system.uuid);
            update_fronter_category(db, pk_client, discord_client, cache, system, &switch).await?;
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
        if let Err(err) = process_system(
            &ctx.services.db,
            &ctx.services.pk,
            &ctx.client,
            &ctx.services.cache,
            system,
        )
        .await
        {
            tracing::warn!("error updating system {}: {}", system.uuid, err);
        }
    }

    Ok(())
}

async fn update_fronters_for_guild(
    client: &Client,
    pk: &PkClient,
    cache: &Cache,
    guild_settings: &ModPkGuildRow,
    category_id: Id<ChannelMarker>,
    members: &[Member],
) -> Result<(), Error> {
    let guild = client
        .guild(*guild_settings.guild_id)
        .await?
        .model()
        .await?;

    let category = client
        .channel(category_id)
        .await
        .map_err(|err| format!("couldn't find category for guild {}: {}", guild.id, err))?
        .model()
        .await?;

    category.guild_id.ok_or_else(|| {
        format!(
            "caetgory {} for guild {} isn't a guild channel",
            category.id, guild.id
        )
    })?;

    super::shared::update_fronter_channels(
        client,
        pk,
        cache,
        guild.clone(),
        guild_settings,
        category,
        Some(members),
    )
    .await
    .map_err(|err| format!("error updating fronters for guild {}: {}", guild.id, err))?;

    tracing::info!("fronters updated in guild {}", guild.id);
    Ok(())
}
