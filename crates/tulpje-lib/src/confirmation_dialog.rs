use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    channel::message::{
        Component, MessageFlags,
        component::{ActionRow, Button, ButtonStyle},
    },
};
use twilight_util::builder::message::ButtonBuilder;

use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    util::{error_message, info_message, success_message, warning_message},
};

#[derive(Debug, Clone, Copy)]
pub enum MessageStyle {
    Success,
    Info,
    Warning,
    Error,
}

pub struct ConfirmationDialog {
    prompt: Vec<Component>,
    cancel_response: Vec<Component>,
    confirm_button: Button,
    cancel_button: Button,
}

impl ConfirmationDialog {
    pub fn builder() -> ConfirmationDialogBuilder {
        ConfirmationDialogBuilder::new()
    }

    pub async fn execute(mut self, ctx: &CommandContext) -> Result<bool, Error> {
        // hardcode the button ids
        self.confirm_button.custom_id = Some("confirm".to_string());
        self.cancel_button.custom_id = Some("cancel".to_string());

        // build the components
        let action_row = ActionRow {
            id: None,
            components: vec![self.confirm_button.into(), self.cancel_button.into()],
        };

        let mut components = self.prompt.clone();
        components.push(action_row.into());

        // get the response from discord
        let response = ctx
            .interaction()
            .update_response(&ctx.event.token)
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(Some(&components))
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
            Some(InteractionData::MessageComponent(interaction)) => {
                match interaction.custom_id.as_str() {
                    "confirm" => Ok(true),
                    "cancel" => {
                        ctx.interaction()
                            .update_response(&ctx.event.token)
                            .flags(MessageFlags::IS_COMPONENTS_V2)
                            .components(Some(&self.cancel_response))
                            .await?;
                        Ok(false)
                    }
                    _ => Err(format!("unknown button id: {}", interaction.custom_id).into()),
                }
            }
            _ => Err(format!(
                "incorrect interaction kind received: {}",
                interaction.kind.kind()
            )
            .into()),
        }
    }
}

#[derive(Default)]
pub struct ConfirmationDialogBuilder {
    prompt: Option<Vec<Component>>,
    cancel_response: Option<Vec<Component>>,

    confirm_button: Option<Button>,
    cancel_button: Option<Button>,
}

impl ConfirmationDialogBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn confirm_button<F>(mut self, style: ButtonStyle, cb: F) -> Self
    where
        F: FnOnce(ButtonBuilder) -> ButtonBuilder,
    {
        self.confirm_button = Some(cb(ButtonBuilder::new(style)).build());
        self
    }

    pub fn cancel_button<F>(mut self, style: ButtonStyle, cb: F) -> Self
    where
        F: FnOnce(ButtonBuilder) -> ButtonBuilder,
    {
        self.cancel_button = Some(cb(ButtonBuilder::new(style)).build());
        self
    }

    pub fn prompt_text(self, style: MessageStyle, text: &str) -> Self {
        self.prompt(&[match style {
            MessageStyle::Success => success_message(text),
            MessageStyle::Info => info_message(text),
            MessageStyle::Warning => warning_message(text),
            MessageStyle::Error => error_message(text),
        }])
    }

    pub fn prompt(mut self, components: &[Component]) -> Self {
        self.prompt = Some(components.to_vec());
        self
    }

    pub fn cancel_text(self, style: MessageStyle, text: &str) -> Self {
        self.cancel_response(&[match style {
            MessageStyle::Success => success_message(text),
            MessageStyle::Info => info_message(text),
            MessageStyle::Warning => warning_message(text),
            MessageStyle::Error => error_message(text),
        }])
    }

    pub fn cancel_response(mut self, components: &[Component]) -> Self {
        self.cancel_response = Some(components.to_vec());
        self
    }

    pub fn build(self) -> ConfirmationDialog {
        ConfirmationDialog {
            prompt: self
                .prompt
                .unwrap_or_else(|| vec![warning_message("### Warning\nAre you sure?")]),
            cancel_response: self
                .cancel_response
                .unwrap_or_else(|| vec![info_message("### Canceled\nAction canceled")]),
            confirm_button: self.confirm_button.unwrap_or_else(|| {
                ButtonBuilder::new(ButtonStyle::Danger)
                    .label("Confirm")
                    .build()
            }),
            cancel_button: self.cancel_button.unwrap_or_else(|| {
                ButtonBuilder::new(ButtonStyle::Secondary)
                    .label("Cancel")
                    .build()
            }),
        }
    }
}
