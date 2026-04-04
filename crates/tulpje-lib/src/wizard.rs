use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tulpje_framework::{
    Error,
    context::{CommandContext, ComponentInteractionContext, ModalContext},
};
use twilight_http::Client;
use twilight_model::{
    application::interaction::{InteractionData, modal::ModalInteractionComponent},
    channel::{
        Message,
        message::{Component, MessageFlags},
    },
    gateway::payload::incoming::InteractionCreate,
    id::{
        Id,
        marker::{ApplicationMarker, GuildMarker, InteractionMarker, MessageMarker},
    },
};

use crate::{context::Services, db::interaction_state};

pub struct WizardContext<T>
where
    T: Clone + Send + Sync,
{
    pub application_id: Id<ApplicationMarker>,
    pub services: Arc<T>,
    pub client: Arc<Client>,

    pub event: InteractionCreate,
    pub guild_id: Option<Id<GuildMarker>>,
    pub interaction_id: Id<InteractionMarker>,
    pub interaction_token: String,
}

impl<T> WizardContext<T>
where
    T: Clone + Send + Sync,
{
    pub fn from_command_context(ctx: &CommandContext<T>) -> Self {
        Self {
            application_id: ctx.application_id,
            services: Arc::clone(&ctx.services),
            client: Arc::clone(&ctx.client),

            event: ctx.event.clone(),
            guild_id: ctx.event.guild_id,
            interaction_id: ctx.event.id,
            interaction_token: ctx.event.token.clone(),
        }
    }

    pub fn from_component_interaction_context(ctx: &ComponentInteractionContext<T>) -> Self {
        Self {
            application_id: ctx.application_id,
            services: Arc::clone(&ctx.services),
            client: Arc::clone(&ctx.client),

            event: ctx.event.clone(),
            guild_id: ctx.event.guild_id,
            interaction_id: ctx.event.id,
            interaction_token: ctx.event.token.clone(),
        }
    }

    pub fn from_modal_context(ctx: &ModalContext<T>) -> Self {
        Self {
            application_id: ctx.application_id,
            services: Arc::clone(&ctx.services),
            client: Arc::clone(&ctx.client),

            event: ctx.event.clone(),
            guild_id: ctx.event.guild_id,
            interaction_id: ctx.event.id,
            interaction_token: ctx.event.token.clone(),
        }
    }

    pub fn get_form_field_text(&self, custom_id: &str) -> Result<String, Error> {
        fn find_text_component(
            component: &ModalInteractionComponent,
            custom_id: &str,
        ) -> Option<String> {
            match component {
                ModalInteractionComponent::Label(label) => match *label.component {
                    ModalInteractionComponent::TextInput(ref input)
                        if input.custom_id == custom_id =>
                    {
                        Some(input.value.clone())
                    }
                    _ => None,
                },
                ModalInteractionComponent::TextInput(input) if input.custom_id == custom_id => {
                    Some(input.value.clone())
                }
                _ => None,
            }
        }

        let event = self.event.data.as_ref().ok_or("no interaction data")?;
        let InteractionData::ModalSubmit(modal) = event else {
            return Err("not a modal interaction".into());
        };

        let value = modal
            .components
            .iter()
            .find_map(|c| find_text_component(c, custom_id))
            .ok_or_else(|| format!("couldn't find text input with custom_id `{custom_id}`"))?;

        Ok(value)
    }

    pub async fn update(&self, components: &[Component]) -> Result<Message, Error> {
        Ok(self
            .client
            .interaction(self.application_id)
            .update_response(&self.interaction_token)
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(Some(components))
            .await
            .map_err(|err| format!("error updating response: {err}"))?
            .model()
            .await
            .map_err(|err| format!("error deserializing response: {err}"))?)
    }
}

#[async_trait::async_trait]
pub trait WizardStep<S>: std::fmt::Debug {
    /// run the step, process inputs, update state, respond to the user, etc.
    async fn run(&self, ctx: &WizardContext<Services>, state: S) -> Result<Option<S>, Error>;
}

pub async fn start_wizard<S>(
    ctx: CommandContext<Services>,
    step: impl WizardStep<S>,
    state: Option<S>,
) -> Result<(), Error>
where
    S: Unpin + Sync + Send + Serialize + for<'de> Deserialize<'de> + Default,
{
    tracing::debug!("starting wizard at step {step:?}");

    // get the message id
    let message_id = ctx
        .interaction()
        .response(&ctx.event.token)
        .await?
        .model()
        .await?
        .id;

    let ctx = WizardContext::from_command_context(&ctx);
    _wizard_step_inner(ctx, step, state.unwrap_or_default(), message_id).await?;
    Ok(())
}

pub async fn handle_component_interaction<S>(
    ctx: ComponentInteractionContext<Services>,
    step: impl WizardStep<S>,
) -> Result<(), Error>
where
    S: Unpin + Sync + Send + Serialize + for<'de> Deserialize<'de> + Default,
{
    tracing::debug!("running wizard step {step:?}");

    let guild_id = ctx.event.guild_id.ok_or("event is missing guild_id")?;
    let message_id = ctx
        .event
        .message
        .as_ref()
        .map(|m| m.id)
        .ok_or("no message_id for ComponentInteraction")?;

    // fetch the state from the database
    let state = interaction_state::get(&ctx.services.db, guild_id, message_id, "TODO", "ALSO_TODO")
        .await?
        .ok_or_else(|| {
            format!("missing interaction_state for message {message_id} in guild {guild_id}")
        })?
        .state
        .0;

    _wizard_step_inner(
        WizardContext::from_component_interaction_context(&ctx),
        step,
        state,
        message_id,
    )
    .await?;

    Ok(())
}

pub async fn handle_modal<S>(
    ctx: ModalContext<Services>,
    step: impl WizardStep<S>,
) -> Result<(), Error>
where
    S: Unpin + Sync + Send + Serialize + for<'de> Deserialize<'de> + Default,
{
    tracing::debug!("running wizard step {step:?}");

    let guild_id = ctx.event.guild_id.ok_or("event is missing guild_id")?;
    let message_id = ctx
        .event
        .message
        .as_ref()
        .map(|m| m.id)
        .ok_or("no message_id for ComponentInteraction")?;

    // fetch the state from the database
    let state = interaction_state::get(&ctx.services.db, guild_id, message_id, "TODO", "ALSO_TODO")
        .await?
        .ok_or_else(|| {
            format!("missing interaction_state for message {message_id} in guild {guild_id}")
        })?
        .state
        .0;

    _wizard_step_inner(
        WizardContext::from_modal_context(&ctx),
        step,
        state,
        message_id,
    )
    .await?;

    Ok(())
}

async fn _wizard_step_inner<S>(
    ctx: WizardContext<Services>,
    step: impl WizardStep<S>,
    state: S,
    message_id: Id<MessageMarker>,
) -> Result<(), Error>
where
    S: Unpin + Sync + Send + Serialize + for<'de> Deserialize<'de>,
{
    let guild_id = ctx.guild_id.ok_or("event is missing guild_id")?;

    match step.run(&ctx, state).await? {
        Some(state) => {
            // store the new state
            interaction_state::set(
                &ctx.services.db,
                guild_id,
                message_id,
                "TODO",
                "ALSO_TODO",
                state,
            )
            .await
            .map_err(|err| format!("error saving interaction state: {err}"))?;
        }
        None => {
            // if no state is returned the interaction is finished, delete the state
            interaction_state::delete(&ctx.services.db, guild_id, message_id, "TODO", "ALSO_TODO")
                .await
                .map_err(|err| format!("error deleting interaction state: {err}"))?;
        }
    }

    Ok(())
}

#[macro_export]
macro_rules! wizard_component {
    ($step_struct:expr $(,)?) => {
        |ctx: $crate::context::ComponentInteractionContext| {
            Box::pin($crate::wizard::handle_component_interaction(
                ctx,
                $step_struct,
            ))
        }
    };
}

#[macro_export]
macro_rules! wizard_modal {
    ($step_struct:expr $(,)?) => {
        |ctx: $crate::context::ModalContext| {
            Box::pin($crate::wizard::handle_modal(ctx, $step_struct))
        }
    };
}
