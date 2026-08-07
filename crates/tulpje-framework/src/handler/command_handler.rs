use std::{future::Future, pin::Pin};

use twilight_model::channel::message::MessageFlags;

use super::super::context::CommandContext;
use crate::{Error, error::UserFacingError, handler::send_internal_handler_error};

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
        // TODO: reWrite, it looks awful like this T_T
        if let Err(err) = (self.func)(ctx.clone()).await {
            if let Some(user_err) = err.downcast_ref::<UserFacingError>() {
                if let Err(inner_err) = ctx
                    .interaction()
                    .update_response(&ctx.event.token)
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(Some(&user_err.components()))
                    .await
                {
                    send_reference(&self.name, ctx, inner_err.into())
                        .await
                        .map_err(|err| format!("error handling internal error: {err}"))?;
                }
            } else {
                send_reference(&self.name, ctx, err)
                    .await
                    .map_err(|err| format!("error handling internal error: {err}"))?;
            }
        }

        Ok(())
    }
}

async fn send_reference<T>(
    command_name: &str,
    ctx: CommandContext<T>,
    err: Error,
) -> Result<(), Error>
where
    T: Clone + Send + Sync,
{
    tracing::error!("error during command `/{command_name}`, sending reference to client: {err}",);

    let Some(chan) = &ctx.event.channel else {
        tracing::warn!("channel on event was empty, can't send error to user");
        return Ok(());
    };

    send_internal_handler_error(chan.id, ctx.meta.uuid, ctx.into())
        .await
        .map_err(|err| format!("error handling internal error: {err}"))?;

    Ok(())
}
