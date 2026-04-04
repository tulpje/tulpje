use std::{future::Future, pin::Pin};

use twilight_model::channel::message::MessageFlags;
use twilight_util::builder::message::{ContainerBuilder, TextDisplayBuilder};

use super::super::context::CommandContext;

use crate::{Error, color};

pub(crate) type CommandFunc<T> =
    fn(CommandContext<T>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

#[derive(Clone)]
pub struct CommandHandler<T: Clone + Send + Sync> {
    pub module: String,
    pub name: String,
    pub func: CommandFunc<T>,
}

impl<T: Clone + Send + Sync> CommandHandler<T> {
    async fn handle_internal_error(&self, ctx: CommandContext<T>, err: Error) -> Result<(), Error> {
        tracing::error!(
            "error during command {}, sending reference to client: {}",
            self.name,
            err
        );

        // TODO: Implement for DMs
        let Some(chan) = &ctx.event.channel else {
            tracing::warn!(event = ?ctx.event, "channel on event was empty, can't send error");
            return Ok(());
        };

        ctx.client
            .create_message(chan.id)
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(&[ContainerBuilder::new()
                .accent_color(Some(*color::roles::RED))
                .component(
                    // TODO: Better way to handle extra error info than, whatever this is
                    TextDisplayBuilder::new(format!(
                        "### Internal Error\n{}\n**Error Code**\n```{}```",
                        std::env::var("TULPJE_EXTRA_ERROR_MESSAGE").unwrap_or_default(),
                        ctx.meta.uuid
                    ))
                    .build(),
                )
                .build()
                .into()])
            .await?;

        Ok(())
    }

    #[tracing::instrument(name="command-handler", skip_all, fields(
        module=self.module,
        name=self.name
    ))]
    pub async fn run(&self, ctx: CommandContext<T>) -> Result<(), Error> {
        if let Err(err) = (self.func)(ctx.clone()).await {
            self.handle_internal_error(ctx, err)
                .await
                .map_err(|err| format!("error handling internal error: {err}"))?;
        }

        Ok(())
    }
}
