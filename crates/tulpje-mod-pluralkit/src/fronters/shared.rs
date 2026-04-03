use std::collections::{HashMap, HashSet};

use chrono::DateTime;
use chrono::NaiveDateTime;
use pkrs_fork::client::PkClient;
use pkrs_fork::client::PluralKitError;
use pkrs_fork::model::Member;
use pkrs_fork::model::PkId;
use serde_either::StringOrStruct;
use tracing::Level;
use tulpje_cache::Cache;
use twilight_http::Client;
use twilight_model::channel::{Channel, ChannelType};
use twilight_model::guild::Guild;
use twilight_model::id::Id;
use twilight_model::id::marker::{ChannelMarker, GuildMarker};

use tulpje_framework::Error;
use uuid::Uuid;

use super::db;
use crate::db::ModPkSystem;
use crate::util::SystemRef;
use crate::util::get_member_name;
use tulpje_lib::{context::CommandContext, responses};

pub(super) async fn get_fronter_channels(
    client: &Client,
    cache: &Cache,
    guild: Id<GuildMarker>,
    cat_id: Id<ChannelMarker>,
) -> Result<Vec<Channel>, Error> {
    // try fetching channels from cache first
    let channel_ids = cache.guild_channels.members(&guild).await?;
    if !channel_ids.is_empty() {
        let mut channels = Vec::new();
        for channel_id in channel_ids {
            // if the channel isn't in the cache log a warning and try to fetch it from discord
            let channel = if let Some(channel) = cache.channels.get(&channel_id).await? {
                channel
            } else {
                match client.channel(channel_id).await {
                    Ok(channel_resp) => match channel_resp.model().await {
                        Ok(channel) => channel,
                        Err(err) => {
                            tracing::warn!(
                                "error deserialising channel {channel_id} for guild {guild}: {err}"
                            );
                            continue;
                        }
                    },
                    Err(err) => {
                        tracing::warn!(
                            "channel {channel_id} in `guild_channels` cache but error occured when fetching from discord: {err}"
                        );
                        continue;
                    }
                }
            };
            if channel
                .parent_id
                .is_some_and(|parent_id| parent_id == cat_id)
            {
                channels.push(channel);
            }
        }

        Ok(channels)
    } else {
        Ok(client
            .guild_channels(guild)
            .await
            .map_err(|err| format!("error fetching guild channels for {guild}: {err}"))?
            .models()
            .await
            .map_err(|err| format!("error deserialising guild channels for {guild}: {err}"))?
            .into_iter()
            .filter(|c| c.parent_id.is_some_and(|parent_id| parent_id == cat_id))
            .collect())
    }
}

/// output additional debugging information to debug issues with fronter order
fn debug_fronter_order(
    guild: &Guild,
    fronter_channels: &[Channel],
    desired_fronters: &[String],
    fronter_pos_map: &HashMap<String, u16>,
) {
    tracing::trace!("fronters for '{}' ({})", guild.name, guild.id);
    tracing::trace!("  fronter_channels");
    let mut sorted_fronter_channels: Vec<_> = fronter_channels.to_vec();
    sorted_fronter_channels.sort_by_key(|c| c.position);

    for channel in sorted_fronter_channels {
        tracing::trace!(
            "    - channel {} ({}) position {:?}",
            channel.name.clone().unwrap_or_default(),
            channel.id,
            channel.position,
        );
    }

    tracing::trace!("  desired_fronters");
    for (position, fronter) in desired_fronters.iter().enumerate() {
        tracing::trace!("    - fronter {fronter} position {position}");
    }

    tracing::trace!("  fronter_pos_map");
    let mut sorted_fronter_pos_map: Vec<(String, u16)> =
        fronter_pos_map.clone().into_iter().collect();
    sorted_fronter_pos_map.sort_by_key(|f| f.1);

    for (fronter, pos) in &sorted_fronter_pos_map {
        tracing::trace!("    - fronter {fronter} position {pos}");
    }
}

pub(super) async fn update_fronter_channels(
    client: &Client,
    cache: &Cache,
    guild: Guild,
    cat: Channel,
    members: &[Member],
) -> Result<(), Error> {
    let fronter_channels = get_fronter_channels(client, cache, guild.id, cat.id).await?;
    let desired_fronters: Vec<_> = members.iter().map(get_member_name).collect();

    let current_fronters: HashSet<String> = fronter_channels
        .iter()
        .map(|c| c.name.clone().expect("guild channels have names"))
        .collect();

    // map fronter names to desired channel positions
    let mut fronter_channel_map: HashMap<String, Channel> = fronter_channels
        .iter()
        .map(|c| {
            (
                c.name.clone().expect("guild channels have names"),
                c.to_owned(),
            )
        })
        .collect();

    let fronter_pos_map: HashMap<String, u16> = desired_fronters
        .iter()
        .enumerate()
        // WARN: could this result in a panic/error? usize into u16
        .map(|(k, v)| (v.to_owned(), k.try_into().unwrap()))
        .collect();

    if tracing::event_enabled!(Level::TRACE) {
        debug_fronter_order(
            &guild,
            &fronter_channels,
            &desired_fronters,
            &fronter_pos_map,
        );
    }

    // calculate wanted changes
    let desired_fronters_set = HashSet::from_iter(desired_fronters);
    let delete_fronters = current_fronters.difference(&desired_fronters_set);
    let create_fronters = desired_fronters_set.difference(&current_fronters);

    // delete old channels
    for fronter in delete_fronters {
        #[expect(
            clippy::indexing_slicing,
            reason = "`delete_fronters` should only contain keys from `fronter_channel_map`"
        )]
        let channel = &fronter_channel_map[fronter];

        client.delete_channel(channel.id).await.map_err(|err| {
            format!(
                "error deleting channel '{}' ({}): {}",
                channel.name.clone().unwrap_or_default(),
                channel.id,
                err
            )
        })?;

        fronter_channel_map.remove(fronter);
    }

    // create new channels
    for fronter in create_fronters {
        let pos = fronter_pos_map
            .get(fronter)
            .expect("couldn't get position for fronter, this should never happen!");

        let channel = client
            .create_guild_channel(guild.id, fronter)
            .position(u64::from(*pos))
            .parent_id(cat.id)
            .kind(ChannelType::GuildVoice)
            .await
            .map_err(|err| format!("error creating fronter channel `{fronter}`: {err}"))?
            .model()
            .await
            .map_err(|err| format!("error deserialising new channel for `{fronter}`: {err}"))?;

        fronter_channel_map.insert(fronter.to_owned(), channel);
    }

    // update channel positions
    for (name, position) in fronter_pos_map {
        let channel = fronter_channel_map
            .get(&name)
            .expect("couldn't get channel from fronter_channel_map, this should never happen!")
            .to_owned();

        if channel.position.is_some_and(|p| p == i32::from(position)) {
            continue;
        }

        client
            .update_channel(channel.id)
            .position(u64::from(position))
            .await
            .map_err(|err| format!("error moving channel `{}` ({}): {}", name, channel.id, err))?;
    }

    Ok(())
}

// check whether a system's front is private and if so
// inform the user with the specified message.
//
// returns true if front is private, with the assumption
// the calling functions early returns after
pub(crate) async fn handle_private_front(
    ctx: &CommandContext,
    system_ref: SystemRef,
    message: &str,
) -> Result<bool, Error> {
    match ctx
        .services
        .pk
        .get_system_fronters(&PkId(system_ref.into()))
        .await
    {
        Ok(_) => Ok(false),
        // 30004 = private front
        Err(PluralKitError::Pk(_, error)) if error.code == 30004 => {
            responses::error(ctx, message).await?;
            Ok(true)
        }
        Err(err) => Err(err.into()),
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum GetSystemFrontersError {
    #[error("fronters for system {0} are private")]
    Private(Uuid),
    #[error(transparent)]
    Other(#[from] Error),
}

pub(crate) struct Fronters {
    pub(crate) members: Vec<Member>,
    pub(crate) timestamp: chrono::NaiveDateTime,
}

pub(crate) async fn get_system_fronters(
    client: &PkClient,
    system_uuid: Uuid,
) -> Result<Option<Fronters>, GetSystemFrontersError> {
    let switch = match client
        .get_system_fronters(&PkId(system_uuid.to_string()))
        .await
    {
        Ok(front) => Ok::<_, GetSystemFrontersError>(front),
        // handle private fronters
        Err(PluralKitError::Pk(_, error))
            // 30004 = private fronters
            if error.code == 30004 =>
        {
            Err(GetSystemFrontersError::Private(system_uuid))
        }
        // directly return any other errors
        Err(err) => Err(GetSystemFrontersError::Other(err.into())),
    }?;

    let Some(switch) = switch else {
        return Ok(None);
    };

    let mut members = Vec::<Member>::new();
    for member in switch.members {
        match member {
            StringOrStruct::String(_) => Err(GetSystemFrontersError::Other(
                format!("system {system_uuid} returned uuids instead of member structs",).into(),
            ))?,
            StringOrStruct::Struct(member) => members.push(member),
        };
    }

    let timestamp = DateTime::from_timestamp(switch.timestamp.to_utc().unix_timestamp(), 0)
        .ok_or_else(|| {
            GetSystemFrontersError::Other(
                format!(
                    "timestamp out of range: {}",
                    switch.timestamp.to_utc().unix_timestamp()
                )
                .into(),
            )
        })?
        .naive_utc();

    Ok(Some(Fronters { members, timestamp }))
}

pub(crate) enum FrontChange {
    Unchanged,
    Changed(Switch),
}

pub(crate) struct Switch {
    pub(crate) fronters: Vec<Member>,
    pub(crate) timestamp: NaiveDateTime,
}

pub(crate) async fn update_system_fronters(
    db: &sqlx::PgPool,
    system: &ModPkSystem,
    client: &PkClient,
) -> Result<FrontChange, GetSystemFrontersError> {
    let fronters: Option<Fronters> = match get_system_fronters(client, system.uuid).await {
        Ok(fronters) => Ok(fronters),
        Err(GetSystemFrontersError::Private(uuid)) => {
            // NOTE: if the fronters are private we still want to update the last_updated
            //       timestamp to avoid getting stuck on trying to update private fronts
            db::update_fronters_timestamp(db, uuid)
                .await
                .map_err(|err| -> Error {
                    format!("error updating fronter timestamp in db for system {uuid}: {err}")
                        .into()
                })?;
            Err(GetSystemFrontersError::Private(uuid))
        }
        Err(err) => Err(err),
    }?;

    let fronter_uuids: Vec<_> = fronters
        .as_ref()
        .map_or_else(Vec::new, |f| f.members.iter().map(|f| f.uuid).collect());

    if db::did_fronters_change(db, system.uuid, &fronter_uuids).await? {
        // update the fronters in the db if they changed
        db::update_fronters(db, system.uuid, &fronter_uuids).await?;
        Ok(FrontChange::Changed(Switch {
            timestamp: fronters
                .as_ref()
                .map_or_else(|| chrono::Utc::now().naive_utc(), |f| f.timestamp),
            fronters: fronters.map_or_else(Vec::new, |f| f.members),
        }))
    } else {
        // otherwise just update the `updated_at` timestamp
        db::update_fronters_timestamp(db, system.uuid).await?;
        Ok(FrontChange::Unchanged)
    }
}
