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
    id::{
        Id,
        marker::{ChannelMarker, GenericMarker},
    },
};

use tulpje_framework::Error;
use tulpje_lib::{
    ConfirmationDialog,
    context::{CommandContext, Services},
    responses,
    util::{find_channel_by_name, handle_channel_from_id, handle_permissions, parse_channel_ref},
};
use twilight_util::builder::message::{ButtonBuilder, TextDisplayBuilder};

use crate::notify::db;

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let channel_name_or_ref = ctx.get_arg_string("channel")?;
    let bot_user = ctx.client.current_user().await?.model().await?;
    let required_permissions =
        Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS;

    let existing_channel = if let Some(channel_id) = parse_channel_ref(&channel_name_or_ref) {
        // handle channel references
        let Some(channel) = handle_channel_from_id(&ctx, guild.id, channel_id).await? else {
            return Ok(());
        };
        Some(channel)
    } else {
        // handle channel names
        find_channel_by_name(
            &ctx.client,
            guild.id,
            &channel_name_or_ref,
            ChannelType::GuildText,
        )
        .await?
    };

    if let Some(configured_channel) = db::get_notify_channel(&ctx.services.db, guild.id).await?
        // don't prompt if they're configuring the same channel again
        && existing_channel
            .as_ref()
            .is_none_or(|chan| chan.id != *configured_channel)
        // show confirmation prompt, and if response is negative return
        && !ConfirmSetup::new(*configured_channel).run(&ctx).await?
    {
        // setup was canceled, return
        return Ok(());
    }

    let channel = if let Some(channel) = existing_channel {
        // if existing channel, check permissions and return if missing
        if !handle_permissions(
            &ctx,
            guild.id,
            bot_user.id,
            Some(&channel),
            required_permissions,
        )
        .await?
        {
            return Ok(());
        }
        channel
    } else {
        // otherwise create the channel
        let permission_overwrites = vec![PermissionOverwrite {
            allow: required_permissions,
            deny: Permissions::empty(),
            id: bot_user.id.cast::<GenericMarker>(),
            kind: PermissionOverwriteType::Member,
        }];
        ctx.client
            .create_guild_channel(guild.id, &channel_name_or_ref)
            .permission_overwrites(&permission_overwrites)
            .kind(ChannelType::GuildText)
            .await?
            .model()
            .await?
    };

    tulpje_lib::db::touch_guild(&ctx.services.db, guild.id).await?;
    db::save_notify_channel(&ctx.services.db, guild.id, channel.id).await?;
    responses::success(
        &ctx,
        &format!(
            "### Success\nTulpje will notify you of front changes in <#{}>",
            channel.id
        ),
    )
    .await?;

    Ok(())
}

/// confirmation dialog for overwriting the existing channel
struct ConfirmSetup {
    channel_id: Id<ChannelMarker>,
}

impl ConfirmSetup {
    fn new(channel_id: Id<ChannelMarker>) -> Self {
        Self { channel_id }
    }
}

#[async_trait::async_trait]
impl ConfirmationDialog<Services> for ConfirmSetup {
    async fn prompt_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![TextDisplayBuilder::new(format!(
            "### Warning\nTulpje already sends notifications to <#{}>, are you sure you want to change it?",
            self.channel_id
        )).build().into()])
    }

    async fn deny_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![TextDisplayBuilder::new(format!(
            "### Canceled\nSetup canceled, keeping <#{}> as notification channel for this server",
            self.channel_id
        )).build().into()])
    }

    async fn confirm_button(&self) -> Result<Button, Error> {
        Ok(ButtonBuilder::new(ButtonStyle::Danger)
            .label("Yes, change it")
            .build())
    }

    async fn deny_button(&self) -> Result<Button, Error> {
        Ok(ButtonBuilder::new(ButtonStyle::Secondary)
            .label("No, cancel")
            .build())
    }
}
