use tulpje_framework::Error;
use tulpje_lib::{context::Services, message_style::MessageStyle, wizard::WizardContext};
use twilight_model::{
    channel::message::{
        EmojiReactionType,
        component::{ActionRow, Button, ButtonStyle, Container, Separator, TextDisplay},
    },
    id::{Id, marker::GuildMarker},
};

use crate::roles::shared::settings::{Settings, settings_display};

pub(crate) async fn view(
    ctx: WizardContext<Services>,
    guild_id: Id<GuildMarker>,
    settings: &Settings,
) -> Result<(), Error> {
    let mut components = vec![
        TextDisplay {
            id: None,
            content: "\
                    Member roles are already set-up in this server, \
                    to change settings use the `/pk role settings` command \
                    or press the button below\
                "
            .into(),
        }
        .into(),
        Separator {
            id: None,
            spacing: None,
            divider: Some(true),
        }
        .into(),
        TextDisplay {
            id: None,
            content: "### Current Settings".into(),
        }
        .into(),
    ];
    components.extend(settings_display(settings));
    components.extend(vec![
        Separator {
            id: None,
            spacing: None,
            divider: Some(true),
        }
        .into(),
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

    ctx.update(&[Container {
        id: None,
        accent_color: Some(Some(MessageStyle::Info.into())),
        spoiler: None,
        components,
    }
    .into()])
        .await?;

    Ok(())
}
