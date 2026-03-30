use tulpje_framework::Error;
use twilight_model::{
    channel::{
        ChannelType,
        message::component::ButtonStyle,
        permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
    },
    guild::Permissions,
    id::marker::GenericMarker,
};

use super::db;
use crate::{
    db::{ModPkSystem, get_guild_settings_for_id, get_system},
    fronters::{db::get_fronter_category, shared::handle_private_front},
    util::SystemRef,
};
use tulpje_lib::{
    confirmation_dialog::{ConfirmationDialogBuilder, MessageStyle},
    context::CommandContext,
    responses,
    util::handle_permissions,
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?;

    tracing::debug!("/pk fronters setup, fetching guild settings {}", guild.id,);
    let Some(guild_settings) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        responses::error(&ctx, "PluralKit module not set-up, please run `/pk setup`").await?;
        return Ok(());
    };

    tracing::debug!(
        "/pk fronters setup, checking permissions for guild {}",
        guild.id,
    );
    let bot_user = ctx.client.current_user().await?.model().await?;
    let required_permissions =
        Permissions::MANAGE_CHANNELS | Permissions::VIEW_CHANNEL | Permissions::CONNECT;
    if !handle_permissions(&ctx, guild.id, bot_user.id, None, required_permissions).await? {
        return Ok(());
    }

    tracing::debug!(
        "/pk fronters setup, checking for existing category for guild {}",
        guild.id,
    );
    // check for an existing category
    if let Some(existing_category) = get_fronter_category(&ctx.services.db, guild.id).await.map_err(|err| format!("couldn't fetch fronter category: {err}"))?
        // and try to fetch the associated system (which should always return one, considering
        // the foreign key constraint
        && let Some(existing_system) =
            get_system(&ctx.services.db, &guild_settings.system_uuid.into()).await.map_err(|err| format!("couldn't fetch system {}: {}", guild_settings.system_uuid, err))?
        // then also get the channel name to show it to the uer
        // TODO: Use cache
        && let Ok(channel_resp) = ctx.client.channel(*existing_category).await
        && let Ok(channel) = channel_resp.model().await
        && let Some(channel_name) = channel.name
        // prompt the user for confirmation to overwrite
        && !handle_overwrite_existing_category(&ctx, &existing_system, &channel_name).await.map_err(|err| format!("error handling confirmation dialog: {err}"))?
    {
        return Ok(());
    }

    let system_ref = SystemRef::Uuid(guild_settings.system_uuid);

    tracing::debug!("/pk fronters setup, fetching system {system_ref}");
    let system = get_system(&ctx.services.db, &system_ref)
        .await
        .map_err(|err| format!("error fetching system {}: {}", system_ref, err))?;
    let display_name = system.map_or_else(|| system_ref.to_string(), |s| s.name.unwrap_or(s.id));

    // inform the user if their front is private
    tracing::debug!("/pk fronters setup, handling potential private front for system {system_ref}");
    if handle_private_front(
        &ctx,
        system_ref.clone(),
        &format!("Front for system `{display_name}` is private, please set it to public to use the fronter list")
    )
    .await?
    {
        return Ok(());
    }

    let category_title = ctx.get_arg_string("title")?;

    // define required permissions
    let permission_overwrites = vec![
        PermissionOverwrite {
            deny: Permissions::VIEW_CHANNEL,
            allow: Permissions::empty(),
            id: guild.id.cast(),
            kind: PermissionOverwriteType::Role,
        },
        PermissionOverwrite {
            allow: required_permissions,
            deny: Permissions::empty(),
            id: bot_user.id.cast::<GenericMarker>(),
            kind: PermissionOverwriteType::Member,
        },
    ];

    tracing::debug!(
        "/pk fronters setup, creating category for guild {}",
        guild.id
    );
    // create the category
    let fronters_category = ctx
        .client
        .create_guild_channel(guild.id, &category_title)
        .permission_overwrites(&permission_overwrites)
        .kind(ChannelType::GuildCategory)
        .await?
        .model()
        .await?;

    // Save category into db
    db::save_fronter_category(&ctx.services.db, guild.id, fronters_category.id).await?;

    // Inform user of success
    responses::success(&ctx, "Fronter category succesfully set-up!").await?;
    Ok(())
}

async fn handle_overwrite_existing_category(
    ctx: &CommandContext,
    system: &ModPkSystem,
    name: &str,
) -> Result<bool, Error> {
    ConfirmationDialogBuilder::new()
        .prompt_text(
            MessageStyle::Warning,
            &format!(
                "### Warning\nTulpje already shows fronters for `{}` under `{}`, are you sure you want to create a new fronter category?",
                system.name.as_ref().unwrap_or(&system.id),
                name
            ),
        )
        .cancel_text(
            MessageStyle::Info,
            &format!(
                "### Canceled\nSetup canceled, keeping `{}` as fronter category for `{}` in this server",
                name,
                system.name.as_ref().unwrap_or(&system.id),
            ),
        )
        .confirm_button(ButtonStyle::Danger, |builder| {
            builder.label("Yes, create new category")
        })
        .cancel_button(ButtonStyle::Secondary, |builder| {
            builder.label("No, cancel")
        })
        .build()
        .execute(ctx)
        .await
}
