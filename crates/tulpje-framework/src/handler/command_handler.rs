use std::{future::Future, pin::Pin};

use super::super::context::CommandContext;
use crate::{Error, handler::send_internal_handler_error};

pub(crate) type CommandFunc<T> =
    fn(CommandContext<T>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

#[derive(Clone)]
pub struct CommandHandler<T: Clone + Send + Sync> {
    pub module: String,
    pub name: String,
    pub func: CommandFunc<T>,
}

impl<T: Clone + Send + Sync> CommandHandler<T> {
    #[tracing::instrument(name="command-handler", skip_all, fields(
        module=self.module,
        name=self.name
    ))]
    pub async fn run(&self, ctx: CommandContext<T>) -> Result<(), Error> {
        if let Err(err) = (self.func)(ctx.clone()).await {
            tracing::error!(
                "error during command `/{}`, sending reference to client: {}",
                self.name,
                err
            );

            let Some(chan) = &ctx.event.channel else {
                tracing::warn!("channel on event was empty, can't send error to user");
                return Ok(());
            };

            send_internal_handler_error(chan.id, ctx.meta.uuid, ctx.into())
                .await
                .map_err(|err| format!("error handling internal error: {err}"))?;
        }

        Ok(())
    }
}
