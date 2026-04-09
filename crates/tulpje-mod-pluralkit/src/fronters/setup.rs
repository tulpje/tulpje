use tulpje_framework::Error;
use twilight_model::{
    channel::{
        ChannelType,
        message::{
            Component,
            component::{Button, ButtonStyle},
        },
        permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
    },
    guild::Permissions,
    id::marker::GenericMarker,
};
use twilight_util::builder::message::{ButtonBuilder, TextDisplayBuilder};

use super::db;
use crate::{
    db::{get_guild_settings_for_id, get_system},
    fronters::{
        db::get_fronter_category,
        shared::{GetSystemFrontersError, get_system_fronters, update_fronter_channels},
    },
    util::SystemRef,
};
use tulpje_lib::{
    ConfirmationDialog,
    context::{CommandContext, Services},
    responses,
    util::handle_permissions,
};

// NOTE: Workaround because | operator isn't const
//       see: https://github.com/bitflags/bitflags/issues/180
const REQUIRED_CATEGORY_PERMISSIONS: Permissions = Permissions::from_bits_truncate(
    Permissions::empty().bits()
        | Permissions::MANAGE_CHANNELS.bits()
        | Permissions::VIEW_CHANNEL.bits()
        | Permissions::CONNECT.bits(),
);

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
    if !handle_permissions(
        &ctx,
        guild.id,
        bot_user.id,
        None,
        REQUIRED_CATEGORY_PERMISSIONS,
    )
    .await?
    {
        return Ok(());
    }

    tracing::debug!(
        "/pk fronters setup, checking for existing category for guild {}",
        guild.id,
    );
    // check for an existing category
    if let Some(existing_category) = get_fronter_category(&ctx.services.db, guild.id).await.map_err(|err| format!("couldn't fetch fronter category: {err}"))?
        // then also get the channel name to show it to the user
        // TODO: Use cache
        && let Ok(channel_resp) = ctx.client.channel(*existing_category).await
        && let Ok(channel) = channel_resp.model().await
        && let Some(channel_name) = channel.name
        // prompt the user for confirmation to overwrite
        && !ConfirmOverwriteCategoryDialog::new(channel_name).run(&ctx).await.map_err(|err| format!("error handling confirmation dialog: {err}"))?
    {
        return Ok(());
    }

    let system_ref = SystemRef::Uuid(guild_settings.system_uuid);

    tracing::debug!("/pk fronters setup, fetching system {system_ref}");
    let Some(system) = get_system(&ctx.services.db, &system_ref)
        .await
        .map_err(|err| format!("error fetching system {}: {}", system_ref, err))?
    else {
        return Err("system {system_ref} missing from database".into());
    };
    let display_name = system.name.unwrap_or(system.id);

    // TODO: Fix horrible deduplication between this and `update_system_fronters`
    let members = match get_system_fronters(&ctx.services.pk, system.uuid).await {
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

            if let Err(err) = db::update_fronters_timestamp(&ctx.services.db, system.uuid).await {
                tracing::warn!(
                    "error updating fronter timestamp in db for system {}: {}",
                    system.uuid,
                    err
                );
            }
            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };

    let category_title = ctx.get_arg_string("title")?;

    tracing::debug!(
        "/pk fronters setup, creating category for guild {}",
        guild.id
    );

    // define required permissions
    let permission_overwrites = vec![
        // deny @everyone connect permissions
        PermissionOverwrite {
            deny: Permissions::CONNECT,
            allow: Permissions::empty(),
            id: guild.id.cast(),
            kind: PermissionOverwriteType::Role,
        },
        // give bot required permissions to operate
        PermissionOverwrite {
            allow: REQUIRED_CATEGORY_PERMISSIONS,
            deny: Permissions::empty(),
            id: bot_user.id.cast::<GenericMarker>(),
            kind: PermissionOverwriteType::Member,
        },
    ];

    // create the category
    let fronters_category = ctx
        .client
        .create_guild_channel(guild.id, &category_title)
        .permission_overwrites(&permission_overwrites)
        .kind(ChannelType::GuildCategory)
        .await?
        .model()
        .await?;

    // update fronters
    update_fronter_channels(&ctx.client, &guild, &fronters_category, &members).await?;

    // Save category into db
    // NOTE: We do this after updating fronter channels so we don't have a race
    //       condition with the auto updating job
    db::save_fronter_category(&ctx.services.db, guild.id, fronters_category.id).await?;

    // Inform user of success
    responses::success(&ctx, "Fronter category succesfully set-up!").await?;
    Ok(())
}

struct ConfirmOverwriteCategoryDialog {
    name: String,
}

impl ConfirmOverwriteCategoryDialog {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[async_trait::async_trait]
impl ConfirmationDialog<Services> for ConfirmOverwriteCategoryDialog {
    async fn prompt_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![
            TextDisplayBuilder::new(format!(
                "### Warning\nTulpje already shows fronters under `{}`, are you sure you want to create a new fronter category?",
                self.name
            ))
            .build()
            .into(),
        ])
    }

    async fn deny_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![
            TextDisplayBuilder::new(format!(
                "### Canceled\nSetup canceled, keeping `{}` as fronter category this server",
                self.name,
            ))
            .build()
            .into(),
        ])
    }

    async fn confirm_button(&self) -> Result<Button, Error> {
        Ok(ButtonBuilder::new(ButtonStyle::Danger)
            .label("Yes, create new category")
            .build())
    }
}
