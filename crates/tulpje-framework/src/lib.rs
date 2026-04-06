use twilight_gateway::Event;
use twilight_model::gateway::payload::incoming::InteractionCreate;

pub use context::{Context, EventContext, InteractionContext};
pub use framework::Framework;
pub use metadata::Metadata;
pub use module::{Module, builder::ModuleBuilder, registry::Registry};

pub mod color;
pub mod constants;
pub mod context;
pub mod framework;
pub mod handler;
pub mod interaction;
pub mod macros;
pub mod metadata;
pub mod module;
pub mod scheduler;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn handle_interaction<T: Clone + Send + Sync + 'static>(
    event: InteractionCreate,
    context: Context<T>,
    meta: &Metadata,
    registry: &Registry<T>,
    processed: twilight_standby::ProcessResults,
) -> Result<(), Error> {
    match interaction::parse(&event, meta.clone(), context) {
        Ok(InteractionContext::Command(ctx)) => {
            let Some(command) = registry.find_command(&ctx.name) else {
                return Err(format!("unknown command /{}", ctx.name).into());
            };

            if let Err(err) = command.run(ctx.clone()).await {
                return Err(format!("error running command /{}: {}", ctx.name, err).into());
            }
        }
        Ok(InteractionContext::ComponentInteraction(ctx)) => {
            let Some(component_interaction) = registry.components.get(&ctx.interaction.custom_id)
            else {
                // twilight-standby already handled this interaction as well, so
                // we don't wanna error for it
                if processed.matched() > 0 {
                    return Ok(());
                }

                // otherwise do error so the missing handler gets logged
                return Err(format!(
                    "no handler for component interaction {}",
                    ctx.interaction.custom_id
                )
                .into());
            };

            if let Err(err) = component_interaction.run(ctx.clone()).await {
                return Err(format!(
                    "error handling component interaction {}: {}",
                    ctx.interaction.custom_id, err
                )
                .into());
            }
        }
        Ok(InteractionContext::Modal(ctx)) => {
            let Some(modal) = registry.modals.get(&ctx.data.custom_id) else {
                // twilight-standby already handled this interaction as well, so
                // we don't wanna error for it
                if processed.matched() > 0 {
                    return Ok(());
                }

                // otherwise do error so the missing handler gets logged
                return Err(format!("no handler for modal {}", ctx.data.custom_id).into());
            };

            if let Err(err) = modal.run(ctx.clone()).await {
                return Err(format!(
                    "error handling component interaction {}: {}",
                    ctx.data.custom_id, err
                )
                .into());
            }
        }
        Err(err) => return Err(format!("error handling interaction: {}", err).into()),
    };

    Ok(())
}

pub async fn handle<T: Clone + Send + Sync + 'static>(
    meta: Metadata,
    ctx: Context<T>,
    registry: &Registry<T>,
    event: Event,
) {
    let processed = ctx.standby.process(&event);

    if let twilight_gateway::Event::InteractionCreate(event) = event.clone()
        && let Err(err) = handle_interaction(*event, ctx.clone(), &meta, registry, processed).await
    {
        tracing::warn!(err);
    }

    if let Some(handlers) = registry.events.get(&event.kind()) {
        tracing::debug!(
            "running {} event handlers for event {:?}",
            handlers.len(),
            event.kind()
        );

        for handler in handlers {
            let event_ctx = EventContext::from_context(ctx.clone(), meta.clone(), event.clone());

            if let Err(err) = handler.run(event_ctx).await {
                tracing::warn!(
                    "error running event handler for event {:?} in module {:?}: {}",
                    handler.module,
                    event.kind(),
                    err
                );
            }
        }
    }
}
