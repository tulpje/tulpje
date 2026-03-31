use std::slice;

use tulpje_framework::{Error, context::CommandContext};
use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    channel::message::{
        Component, MessageFlags,
        component::{ActionRow, Button, ButtonStyle, Container, SeparatorSpacingSize},
    },
};
use twilight_util::builder::message::{ButtonBuilder, SeparatorBuilder, TextDisplayBuilder};

use crate::message_style::MessageStyle;

#[async_trait::async_trait]
pub trait ConfirmationDialog<T: Clone + Send + Sync> {
    const PROMPT_STYLE: MessageStyle = MessageStyle::Warning;
    const DENY_STYLE: MessageStyle = MessageStyle::Info;

    async fn prompt_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![
            TextDisplayBuilder::new("### Warning\nAre you sure?")
                .build()
                .into(),
        ])
    }

    async fn deny_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![
            TextDisplayBuilder::new("### Canceled\nAction canceled")
                .build()
                .into(),
        ])
    }

    async fn confirm_button(&self) -> Result<Button, Error> {
        Ok(ButtonBuilder::new(ButtonStyle::Danger)
            .label("Confirm")
            .build())
    }

    async fn deny_button(&self) -> Result<Button, Error> {
        Ok(ButtonBuilder::new(ButtonStyle::Secondary)
            .label("Cancel")
            .build())
    }

    /// method that executes when the user denies the prompt
    ///
    /// Should result `false` if the calling function should early return, otherwise
    /// true. Usually this is `false` when the user denies the prompt
    async fn deny(&self, ctx: &CommandContext<T>) -> Result<bool, Error> {
        let deny_container = Container {
            id: None,
            spoiler: None,

            accent_color: Some(Some(Self::DENY_STYLE.into())),
            components: self.deny_message().await?,
        };

        ctx.interaction()
            .update_response(&ctx.event.token)
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(Some(slice::from_ref(&deny_container.into())))
            .await?;

        Ok(false)
    }

    /// method to execute when the user confirms the prompt
    ///
    /// Should result `true` if the calling function should continue, otherwise
    /// `false`. Usually this is `true` when the user confirms the prompt
    async fn confirm(&self, _ctx: &CommandContext<T>) -> Result<bool, Error> {
        Ok(true)
    }

    async fn run(&self, ctx: &CommandContext<T>) -> Result<bool, Error> {
        let mut confirm_button = self.confirm_button().await?;
        let mut deny_button = self.deny_button().await?;

        // set button IDs if they're missing
        let confirm_id = confirm_button
            .custom_id
            .get_or_insert_with(|| "confirm".to_string())
            .clone();
        let deny_id = deny_button
            .custom_id
            .get_or_insert_with(|| "deny".to_string())
            .clone();

        // build the components
        let action_row = ActionRow {
            id: None,
            components: vec![confirm_button.into(), deny_button.into()],
        };

        let mut components = self.prompt_message().await?.clone();
        components.push(
            SeparatorBuilder::new()
                .divider(true)
                .spacing(SeparatorSpacingSize::Large)
                .build()
                .into(),
        );
        components.push(action_row.into());

        let prompt_container = Container {
            id: None,
            spoiler: None,

            accent_color: Some(Some(Self::PROMPT_STYLE.into())),
            components,
        };

        // get the response from discord
        let response = ctx
            .interaction()
            .update_response(&ctx.event.token)
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(Some(slice::from_ref(&prompt_container.into())))
            .await?
            .model()
            .await?;

        // wait for user interaction
        let interaction = ctx
            .standby
            .wait_for_component(response.id, |_: &Interaction| true)
            .await?;

        // handle interaction data
        match interaction.data {
            Some(InteractionData::MessageComponent(interaction)) => match interaction.custom_id {
                custom_id if custom_id == *confirm_id => self.confirm(ctx).await,
                custom_id if custom_id == *deny_id => self.deny(ctx).await,
                _ => Err(format!("unknown button id: {}", interaction.custom_id).into()),
            },
            _ => Err(format!(
                "incorrect interaction kind received: {}",
                interaction.kind.kind()
            )
            .into()),
        }
    }
}
