use tulpje_framework::Error;
use tulpje_lib::{context::Services, message_style::MessageStyle, wizard::WizardContext};
use twilight_model::channel::message::component::{
    ActionRow, ButtonStyle, Container, Separator, SeparatorSpacingSize, TextDisplay,
};
use twilight_util::builder::message::ButtonBuilder;

use crate::roles::{
    role_limit::{RoleLimitData, near_role_limit_message},
    setup::custom_ids,
};

pub(crate) async fn view(ctx: &WizardContext<Services>, data: &RoleLimitData) -> Result<(), Error> {
    ctx.update(&[Container {
        id: None,
        spoiler: None,

        accent_color: Some(Some(MessageStyle::Warning.into())),
        components: vec![
            TextDisplay {
                id: None,
                content: near_role_limit_message(data),
            }
            .into(),
            Separator {
                id: None,
                divider: Some(true),
                spacing: Some(SeparatorSpacingSize::Large),
            }
            .into(),
            ActionRow {
                id: None,
                components: vec![
                    ButtonBuilder::new(ButtonStyle::Primary)
                        .label("Yes, continue")
                        .custom_id(custom_ids::COMPONENT_NEAR_ROLE_LIMIT_ACCEPT)
                        .build()
                        .into(),
                    ButtonBuilder::new(ButtonStyle::Danger)
                        .label("No, cancel")
                        .custom_id(custom_ids::COMPONENT_NEAR_ROLE_LIMIT_DENY)
                        .build()
                        .into(),
                ],
            }
            .into(),
        ],
    }
    .into()])
        .await?;
    Ok(())
}

pub(crate) async fn deny_view(ctx: &WizardContext<Services>) -> Result<(), Error> {
    ctx.update(&[Container {
        id: None,
        spoiler: None,

        accent_color: Some(Some(MessageStyle::Info.into())),
        components: vec![
            TextDisplay {
                id: None,
                content: "### Role Setup Canceled".into(),
            }
            .into(),
        ],
    }
    .into()])
        .await?;

    Ok(())
}
