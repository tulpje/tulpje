use std::{future::Future, pin::Pin};

use twilight_gateway::EventType;

use super::super::context::EventContext;
use crate::Error;

pub(crate) type EventFunc<T> =
    fn(EventContext<T>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

#[derive(Clone)]
pub struct EventHandler<T: Clone + Send + Sync> {
    pub module: String,
    pub event: EventType,
    pub func: EventFunc<T>,
}

impl<T: Clone + Send + Sync> EventHandler<T> {
    #[tracing::instrument(name="event-handler", skip_all, fields(
        module=self.module,
        event=?self.event
    ))]
    pub async fn run(&self, ctx: EventContext<T>) -> Result<(), Error> {
        // can add more handling/parsing/etc here in the future
        (self.func)(ctx).await
    }
}
