use std::{future::Future, pin::Pin};

use super::super::context::ModalContext;
use crate::Error;

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
        // can add more handling/parsing/etc here in the future
        (self.func)(ctx).await
    }
}
