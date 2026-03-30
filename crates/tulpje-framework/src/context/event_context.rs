use std::sync::Arc;

use twilight_gateway::Event;
use twilight_http::Client;
use twilight_model::id::{Id, marker::ApplicationMarker};
use twilight_standby::Standby;

use crate::{Context, Metadata};

#[derive(Clone, Debug)]
pub struct EventContext<T: Clone + Send + Sync> {
    pub meta: Metadata,
    pub application_id: Id<ApplicationMarker>,
    pub services: Arc<T>,
    pub client: Arc<Client>,
    pub standby: Arc<Standby>,

    pub event: Event,
}

impl<T: Clone + Send + Sync> EventContext<T> {
    pub fn from_context(ctx: Context<T>, meta: Metadata, event: Event) -> Self {
        Self {
            application_id: ctx.application_id,
            client: ctx.client,
            services: ctx.services,
            standby: ctx.standby,

            meta,
            event,
        }
    }
}
