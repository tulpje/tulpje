use std::collections::HashSet;

use tulpje_framework::Error;
use tulpje_lib::{
    context::CommandContext,
    db::guild_settings::GuildSettingsResolver,
    responses,
    wizard::{WizardContext, start_wizard},
};
use twilight_model::id::{Id, marker::RoleMarker};

use crate::{
    db::get_guild_settings_for_id,
    roles::{
        db,
        role_limit::{RoleLimitData, RoleLimitResult, over_role_limit_message},
        setup::{
            view::already_setup,
            wizard::{PromptLegacyRolesCleanup, PromptNearRoleLimit, PromptRoleSuffix, SetupState},
        },
        shared::handle_get_system_members,
    },
};

mod components;
pub(crate) mod custom_ids;
mod view;
pub(crate) mod wizard;

/// shows the initial prompt when `/pk role setup` is executed
pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral()
        .await
        .map_err(|err| format!("failed to defer: {err}"))?;

    tracing::debug!("trying to fetch existing guild settings");
    let Some(guild_settings) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        // TODO: Add button to start PluralKit setup
        responses::error(&ctx, "PluralKit module not set-up, please run `/pk setup`").await?;
        return Ok(());
    };

    let settings_store = GuildSettingsResolver::new(guild.id, "pluralkit", "roles".into());

    // if roles have been configured before show the current settings and instructions
    // to change them
    tracing::debug!("trying to fetch existing role settings");
    if let Some(settings) = settings_store.get(&ctx.services.db).await? {
        already_setup::view(
            WizardContext::from_command_context(&ctx),
            guild.id,
            &settings.data.0,
        )
        .await?;
        return Ok(());
    }

    // try to fetch system members, let user know if we can't
    // TODO: Offer user the options to:
    //          * change system
    //          * retry after changing privacy
    //          * cancel
    tracing::debug!("trying to fetch system members");
    let Some(system_members) =
        handle_get_system_members(&ctx, &ctx.services.pk, &guild_settings.system_uuid.into())
            .await?
    else {
        return Ok(());
    };

    // fetch saved member roles
    let member_roles: HashSet<Id<RoleMarker>> = db::get_guild_roles(&ctx.services.db, guild.id)
        .await?
        .iter()
        .map(|r| *r.role_id)
        .collect();

    let legacy_roles: Vec<_> = guild
        .roles
        .iter()
        .filter(|r| r.name.ends_with(" (Alter)") && !member_roles.contains(&r.id))
        .collect();

    let role_limit_data = RoleLimitData::new(
        guild.roles.len(),
        member_roles.len(),
        legacy_roles.len(),
        system_members.len(),
    );

    let state = SetupState::with_member_data(&system_members).map_err(|err| {
        format!(
            "error during `SetupState::with_member_data` \
            did PluralKit return an invalid color?: {err}"
        )
    })?;

    tracing::debug!("checking role limits");
    match role_limit_data.check() {
        RoleLimitResult::Over => {
            // over role limit let user know and abort
            responses::error(&ctx, &over_role_limit_message(&role_limit_data)).await?;
            return Ok(());
        }
        RoleLimitResult::Near => {
            // near role limit, ask user if they're okay with that
            start_wizard(ctx, PromptNearRoleLimit::new(role_limit_data), Some(state)).await?;
            return Ok(());
        }
        RoleLimitResult::Ok => {}
    }

    if !legacy_roles.is_empty() {
        start_wizard(
            ctx,
            PromptLegacyRolesCleanup,
            Some(SetupState {
                legacy_roles: role_limit_data.legacy_member_roles,
                ..state
            }),
        )
        .await?;
        return Ok(());
    }

    start_wizard(ctx, PromptRoleSuffix, Some(state)).await?;
    Ok(())
}
