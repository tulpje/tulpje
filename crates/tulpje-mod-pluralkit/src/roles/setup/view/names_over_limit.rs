use pluralizer::pluralize;
use rand::prelude::IndexedRandom as _;
use tulpje_framework::Error;
use tulpje_lib::{context::Services, message_style::MessageStyle, wizard::WizardContext};
use twilight_model::channel::message::component::{
    ActionRow, ButtonStyle, Container, Separator, SeparatorSpacingSize, TextDisplay,
};
use twilight_util::builder::message::ButtonBuilder;

use crate::{
    roles::{constants::DISCORD_MAX_ROLE_NAME_LENGTH, setup::custom_ids},
    util::discord_ellipsize,
};

pub(crate) async fn view(
    ctx: &WizardContext<Services>,
    names: &[&String],
    suffix: &str,
) -> Result<(), Error> {
    let mut rng: rand::rngs::SmallRng = rand::make_rng();
    let suffix_len = suffix.encode_utf16().count();
    let names_noun = pluralize("member name", names.len() as isize, true);

    ctx.update(&[Container {
        id: None,
        spoiler: None,

        accent_color: Some(Some(MessageStyle::Warning.into())),
        components: vec![
            TextDisplay {
                id: None,
                content: format!(
                    "### Warning\n{names_noun} are over the discord limit of \
                    {DISCORD_MAX_ROLE_NAME_LENGTH} when adding the suffix.\n\
                    Would you like Tulpje to shorten them, like in the examples below?"
                ),
            }
            .into(),
            TextDisplay {
                id: None,
                content: names
                    .sample(&mut rng, 5)
                    .map(|name| {
                        format!(
                            "- {}{}",
                            discord_ellipsize(name, DISCORD_MAX_ROLE_NAME_LENGTH - suffix_len, "…"),
                            suffix,
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
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
                        .label("Yes, shorten names")
                        .custom_id(custom_ids::COMPONENT_NAMES_OVER_LIMIT_ACCEPT)
                        .build()
                        .into(),
                    ButtonBuilder::new(ButtonStyle::Danger)
                        .label("No, change suffix")
                        .custom_id(custom_ids::COMPONENT_NAMES_OVER_LIMIT_DENY)
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
