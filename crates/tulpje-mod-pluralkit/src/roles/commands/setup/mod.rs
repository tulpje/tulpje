use tulpje_cache::models::role;
use tulpje_framework::{Error, InteractionContext};
use tulpje_lib::{
    context::{CommandContext, Services},
    db::guild_settings::GuildSettingsResolver,
    message_style::MessageStyle,
    responses,
};
use twilight_model::{
    channel::message::{
        Component, EmojiReactionType, MessageFlags,
        component::{ActionRow, Button, ButtonStyle, Container, Separator, TextDisplay},
    },
    id::{Id, marker::GuildMarker},
};
use twilight_util::builder::message::TextDisplayBuilder;

use self::state::SetupState;
use crate::{
    db::get_guild_settings_for_id,
    roles::{
        db,
        role_limit::{RoleCommand, RoleLimitHandler, RoleLimitResult},
        settings::Settings,
        shared::{handle_get_system_members, settings_display},
    },
};

mod state;

/// shows the initial prompt when `/pk role setup` is executed
pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    // NOTE: We defer ephemeral because this command _does_ store sensitive data
    ctx.defer_ephemeral().await?;

    tracing::debug!("trying to fetch existing guild settings");
    let Some(guild_settings) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        // TODO: inform user they haven't set up the module, and offer to run the main setup
        return Ok(());
    };

    let settings_store = GuildSettingsResolver::new(guild.id, "pluralkit", "roles".into());

    // if roles have been configured before show the current settings and instructions
    // to change them
    tracing::debug!("trying to fetch existing role settings");
    if let Some(settings) = settings_store.get(&ctx.services.db).await? {
        ctx.interaction()
            .update_response(&ctx.event.token)
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(Some(&already_setup_component(guild.id, &settings.data)))
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
    let member_roles = db::get_guild_roles(&ctx.services.db, guild.id).await?;

    tracing::debug!("checking role limits");
    if !RoleLimitHandler::new(&guild, &member_roles, &system_members)
        .handle(&ctx, RoleCommand::Setup)
        .await?
    {
        // role limit handler couldn't progress for whatever reason, return
        return Ok(());
    }

    let mut state = SetupState::default();
    Ok(())
}

/// handles any followup interactions after the initial setup
pub(crate) async fn handle_interaction(ctx: InteractionContext<Services>) -> Result<(), Error> {
    Ok(())
}

fn already_setup_component(guild_id: Id<GuildMarker>, settings: &Settings) -> Vec<Component> {
    let info_text: Component = TextDisplayBuilder::new(
        "\
            Member roles are already set-up in this server, \
            to change settings use the `/pk role settings` command \
            or press the button below\
        ",
    )
    .build()
    .into();

    let divider = Separator {
        id: None,
        spacing: None,
        divider: Some(true),
    };

    let mut components = vec![
        info_text,
        divider.clone().into(),
        TextDisplay {
            id: None,
            content: "### Current Settings".into(),
        }
        .into(),
    ];
    components.extend(settings_display(settings));
    components.extend(vec![
        divider.into(),
        ActionRow {
            id: None,
            components: vec![
                Button {
                    id: None,
                    sku_id: None,
                    url: None,

                    disabled: false,

                    style: ButtonStyle::Primary,
                    label: Some("Settings".into()),
                    emoji: EmojiReactionType::Unicode {
                        name: "🔧".into()
                    }
                    .into(),
                    custom_id: format!("{guild_id}-pluralkit-setup-role-settings").into(),
                }
                .into(),
            ],
        }
        .into(),
    ]);

    vec![
        Container {
            id: None,
            accent_color: Some(Some(MessageStyle::Info.into())),
            spoiler: None,
            components,
        }
        .into(),
    ]
}
