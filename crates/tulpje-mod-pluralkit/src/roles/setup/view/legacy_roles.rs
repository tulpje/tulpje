use pluralizer::pluralize;
use tulpje_framework::Error;
use tulpje_lib::{context::Services, message_style::MessageStyle, wizard::WizardContext};
use twilight_model::channel::message::component::{
    ActionRow, ButtonStyle, Container, Separator, SeparatorSpacingSize, TextDisplay,
};
use twilight_util::builder::message::ButtonBuilder;

use crate::roles::setup::custom_ids;

pub(crate) async fn view(
    ctx: &WizardContext<Services>,
    legacy_role_count: usize,
) -> Result<(), Error> {
    let legacy_role_noun = pluralize(
        "potential old member role",
        legacy_role_count as isize,
        true,
    );

    ctx.update(&[Container {
        id: None,
        spoiler: None,

        accent_color: Some(Some(MessageStyle::Warning.into())),
        components: vec![
            TextDisplay {
                id: None,
                content: format!(
                    "Tulpje has detected {legacy_role_noun} \
                    (ones with ` (Alter)` in their name), \
                    would you like it to clean those up?",
                ),
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
                        .label("Yes, delete them")
                        .custom_id(custom_ids::COMPONENT_LEGACY_ROLE_CLEANUP_ACCEPT)
                        .build()
                        .into(),
                    ButtonBuilder::new(ButtonStyle::Danger)
                        .label("No, keep")
                        .custom_id(custom_ids::COMPONENT_LEGACY_ROLE_CLEANUP_DENY)
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
