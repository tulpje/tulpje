use std::collections::{HashMap, HashSet};

use pkrs_fork::model::Member;
use tulpje_cache::Cache;
use tulpje_lib::ConfirmationDialog as _;
use twilight_http::Client;
use twilight_model::guild::{Guild, Role};
use twilight_model::id::Id;
use twilight_model::id::marker::{GuildMarker, RoleMarker, UserMarker};

use tulpje_framework::Error;
use tulpje_lib::{context::CommandContext, responses};
use uuid::Uuid;

use crate::roles::constants::{DISCORD_ROLE_LIMIT, REMAINING_ROLE_WARNING};
use crate::roles::prompts::{ConfirmUpdatePrompt, NearRoleLimitWarningPrompt, role_change_message};
use crate::roles::shared::handle_get_system_members;
use crate::roles::update_stats::{UpdateCounts, UpdateStats};
use crate::{
    db::get_guild_settings_for_id,
    util::{SystemRef, get_member_name, pk_color_to_discord},
};

fn role_limit_message(member_count: usize, existing_role_count: usize) -> String {
    let combined_count = member_count + existing_role_count;
    let over_limit = combined_count.saturating_sub(DISCORD_ROLE_LIMIT);
    let existing_role_noun = if existing_role_count == 1 {
        "role"
    } else {
        "roles"
    };
    let member_noun = if member_count == 1 {
        "member"
    } else {
        "members"
    };
    let combined_noun = if combined_count == 1 { "role" } else { "roles" };

    format!(
        "### Error\n\
        This server currently has {existing_role_count} {existing_role_noun}, \
        adding roles for {member_count} sytem {member_noun} would leave you with \
        {combined_count} {combined_noun} which is {over_limit} more than \
        discord's limit of {DISCORD_ROLE_LIMIT} roles"
    )
}

async fn handle_update_success_message(
    ctx: &CommandContext,
    counts: &UpdateCounts,
) -> Result<(), Error> {
    responses::success(
        ctx,
        &format!(
            "### Member Roles Updated\n{}",
            role_change_message(counts, "")
        ),
    )
    .await?;

    Ok(())
}

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?; // delay responding and make reply ephemeral

    let Some(gs) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        responses::error(
            &ctx,
            "### Error\nPluralKit module not set-up, please run `/pk setup`",
        )
        .await?;
        return Ok(());
    };
    let system_ref = SystemRef::Uuid(gs.system_uuid);
    let token = ctx.get_arg_string_optional("token")?;

    // fetch members from PluralKit
    let Some(members) = handle_get_system_members(
        &ctx,
        &ctx.services.pk.with_token(token.unwrap_or_default()),
        &system_ref,
    )
    .await?
    else {
        return Ok(());
    };

    // get current and desired roles
    let current_role_map = get_current_roles(&guild);
    let desired_role_map = get_desired_roles(&members);

    // get statistics for role limits
    let total_guild_roles = guild.roles.len();
    let roles_without_member_roles = total_guild_roles - current_role_map.len();
    let system_member_count = members.len();
    let total_roles_with_members = system_member_count + roles_without_member_roles;

    // inform user that updating member roles would surpass discord's role limit
    if total_roles_with_members > DISCORD_ROLE_LIMIT {
        responses::error(
            &ctx,
            &role_limit_message(system_member_count, roles_without_member_roles),
        )
        .await?;
        return Ok(());
    }

    // prompt the user if they're ok with being close to the role limit
    if total_roles_with_members >= DISCORD_ROLE_LIMIT - REMAINING_ROLE_WARNING
        && !NearRoleLimitWarningPrompt::new(system_member_count, roles_without_member_roles)
            .run(&ctx)
            .await?
    {
        // user canceled
        return Ok(());
    }

    // get current and desired assigned roles for user
    let current_user_roles =
        get_user_roles(&ctx.client, &ctx.services.cache, *gs.guild_id, *gs.user_id).await?;

    let current_user_role_names: HashSet<_> = current_user_roles
        .iter()
        .filter(|r| r.name.ends_with("(Alter)"))
        .map(|r| r.name.clone())
        .collect();
    let desired_user_role_names: HashSet<_> = desired_role_map.keys().cloned().collect();
    let missing_user_role_names: Vec<_> = desired_user_role_names
        .difference(&current_user_role_names)
        .collect();

    let ops = get_role_ops(&current_role_map, &desired_role_map);

    // aggregate stats
    let (create, delete, update) = ops
        .iter()
        .fold((0, 0, 0), |(created, deleted, updated), op| match op {
            ChangeOperation::Create { .. } => (created + 1, deleted, updated),
            ChangeOperation::Delete { .. } => (created, deleted + 1, updated),
            ChangeOperation::Update { .. } => (created, deleted, updated + 1),
        });

    let mut update_stats =
        UpdateStats::new(create, update, delete, missing_user_role_names.len() as u16);

    if update_stats.total.sum() == 0 {
        responses::info(&ctx, "Member roles are already up-to-date").await?;
        return Ok(());
    }

    // prompt user if listed changes are okay
    if !ConfirmUpdatePrompt::new(update_stats.total.clone())
        .run(&ctx)
        .await?
    {
        // user canceled
        return Ok(());
    }

    let mut role_name_id_map: HashMap<String, Id<RoleMarker>> = current_role_map
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.role_id
                    .expect("get_current_roles always assigns id: Some(...)"),
            )
        })
        .collect();

    // TODO: actually handle errors
    // TODO: set mention permissions?
    for (idx, op) in ops.iter().enumerate() {
        match op {
            ChangeOperation::Update { id, color } => {
                ctx.client
                    .update_role(guild.id, *id)
                    .color(Some(*color))
                    .await
                    .map_err(|err| {
                        format!("error updating role {} in guild {}: {}", id, guild.id, err)
                    })?;

                update_stats.done.update += 1;
                tracing::debug!("updated role {} in guild {}", id, guild.id);
            }
            ChangeOperation::Create {
                uuid: member_uuid,
                name,
                color,
            } => {
                let role = ctx
                    .client
                    .create_role(guild.id)
                    .name(name)
                    .color(*color)
                    .await
                    .map_err(|err| {
                        format!(
                            "error creating role for member {} in guild {}: {}",
                            member_uuid, guild.id, err
                        )
                    })?
                    .model()
                    .await
                    .map_err(|err| {
                        format!(
                            "error parsing role for member {} in guild {}: {}",
                            member_uuid, guild.id, err
                        )
                    })?;

                role_name_id_map.insert(name.clone(), role.id);

                update_stats.done.create += 1;
                tracing::debug!("created role {} in guild {}", role.id, guild.id);
            }
            ChangeOperation::Delete { id } => {
                ctx.client.delete_role(guild.id, *id).await.map_err(|err| {
                    format!("error deleting role {} in guild {}: {}", id, guild.id, err)
                })?;

                update_stats.done.delete += 1;
                tracing::debug!("deleted role {} in {}", id, guild.id);
            }
        };

        // update user progress every 10 actions
        if idx % 10 == 0 {
            update_role_progress(&ctx, &update_stats).await;
        }
    }

    for (idx, missing_role_name) in missing_user_role_names.iter().enumerate() {
        let Some(role_id) = role_name_id_map.get(*missing_role_name) else {
            tracing::warn!("couldn't get role id from `role_name_id_map` for {missing_role_name}");
            continue;
        };

        ctx.client
            .add_guild_member_role(*gs.guild_id, *gs.user_id, *role_id)
            .await
            .map_err(|err| {
                format!("error assigning role {missing_role_name} ({role_id}): {err}")
            })?;

        update_stats.done.assign += 1;
        tracing::debug!(
            "assigned role {} to user {} in guild {}",
            role_id,
            gs.user_id,
            gs.guild_id
        );

        // update user progress every 10 actions
        if idx % 10 == 0 {
            update_role_progress(&ctx, &update_stats).await;
        }
    }

    // send success message to user
    handle_update_success_message(&ctx, &update_stats.done).await?;

    Ok(())
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct MemberRole {
    role_id: Option<Id<RoleMarker>>,
    uuid: Option<Uuid>,
    name: String,
    color: u32,
}

enum ChangeOperation {
    Create {
        name: String,
        uuid: Uuid,
        color: u32,
    },
    Delete {
        id: Id<RoleMarker>,
    },
    Update {
        id: Id<RoleMarker>,
        color: u32,
    },
}

async fn update_role_progress(ctx: &CommandContext, stats: &UpdateStats) {
    let mut parts = Vec::<(u16, u16, &'static str)>::new();
    if stats.total.create > 0 {
        parts.push((stats.done.create, stats.total.create, "created"));
    }
    if stats.total.update > 0 {
        parts.push((stats.done.update, stats.total.update, "updated"));
    }
    if stats.total.delete > 0 {
        parts.push((stats.done.delete, stats.total.delete, "deleted"));
    }
    if stats.total.assign > 0 {
        parts.push((stats.done.assign, stats.total.assign, "assigned"));
    }

    if let Err(err) = responses::info(
        ctx,
        &format!(
            "### Updating...\n{}",
            parts
                .into_iter()
                .map(|(done, total, verb)| format!("{done}/{total} {verb}"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )
    .await
    {
        tracing::warn!("error updating role update progress: {err}");
    }
}

// TODO: Persist updated info in the cache
async fn get_user_roles(
    client: &Client,
    cache: &Cache,
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
) -> Result<Vec<Role>, Error> {
    let member_roles = if let Some(member) = cache.members.get(&(guild_id, user_id)).await? {
        member.roles
    } else {
        client
            .guild_member(guild_id, user_id)
            .await?
            .model()
            .await?
            .roles
    };

    let mut roles = Vec::new();
    for role_id in member_roles {
        roles.push(if let Some(role) = cache.roles.get(&role_id).await? {
            role.inner()
        } else {
            client.role(guild_id, role_id).await?.model().await?
        });
    }

    Ok(roles)
}

fn get_desired_roles(members: &[Member]) -> HashMap<String, MemberRole> {
    members
        .iter()
        .map(|m| MemberRole {
            role_id: None,
            uuid: Some(m.uuid),
            name: format!(
                "{} (Alter)",
                get_member_name(m)
                    .split(" (") // Remove parenthesised pronouns ' (she/her)' and such
                    .next() // get the first part of the split string
                    .unwrap()
            ),
            color: pk_color_to_discord(m.color.clone()),
        })
        .map(|r| (r.name.clone(), r))
        .collect()
}

fn get_current_roles(guild: &Guild) -> HashMap<String, MemberRole> {
    guild
        .roles
        .iter()
        .filter(|v| v.name.ends_with(" (Alter)"))
        .map(|v| MemberRole {
            role_id: Some(v.id),
            uuid: None,
            name: v.name.clone(),
            color: v.colors.primary_color,
        })
        .map(|v| (v.name.clone(), v))
        .collect()
}

fn get_role_ops(
    current_roles: &HashMap<String, MemberRole>,
    desired_roles: &HashMap<String, MemberRole>,
) -> Vec<ChangeOperation> {
    let all_roles: HashSet<&String> = current_roles.keys().chain(desired_roles.keys()).collect();

    all_roles
        .into_iter()
        .filter_map(|role| {
            match (current_roles.get(role), desired_roles.get(role)) {
                // Update, only if color changed
                (Some(current), Some(desired)) => {
                    (current.color != desired.color).then(|| ChangeOperation::Update {
                        id: current.role_id.unwrap(),
                        color: desired.color,
                    })
                }
                // Create
                (None, Some(desired)) => Some(ChangeOperation::Create {
                    name: desired.name.clone(),
                    uuid: desired.uuid.unwrap_or_default(),
                    color: desired.color,
                }),
                // Delete
                (Some(current), None) => Some(ChangeOperation::Delete {
                    id: current.role_id.unwrap(),
                }),
                // Shit got fucked up aaaa
                (None, None) => panic!("current and desired are both None, shouldn't happen"),
            }
        })
        .collect()
}
