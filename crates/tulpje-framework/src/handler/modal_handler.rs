use std::{future::Future, pin::Pin};

use super::super::context::ModalContext;
use crate::{Error, handler::send_internal_handler_error};

pub(crate) type ModalFunc<T> =
    fn(ModalContext<T>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

#[derive(Clone)]
pub struct ModalHandler<T: Clone + Send + Sync> {
    pub module: String,
    pub custom_id: String,
    pub func: ModalFunc<T>,
}

impl<T: Clone + Send + Sync> ModalHandler<T> {
    #[tracing::instrument(name="modal-handler", skip_all, fields(
        module=self.module,
        custom_id=self.custom_id
    ))]
    pub async fn run(&self, ctx: ModalContext<T>) -> Result<(), Error> {
        if let Err(err) = (self.func)(ctx.clone()).await {
            tracing::error!(
                "error during modal `{}`, sending reference to client: {}",
                self.custom_id,
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
