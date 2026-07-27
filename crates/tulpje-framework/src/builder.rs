use std::sync::Arc;

use twilight_http::Client;
use twilight_model::id::{Id, marker::ApplicationMarker};

use crate::{Framework, Registry, framework::SetupFunc};

#[derive(Clone)]
pub struct FrameworkBuilder<T: Clone + Send + Sync> {
    registry: Arc<Registry<T>>,
    client: Arc<Client>,
    app_id: Id<ApplicationMarker>,
    user_data: Arc<T>,

    setup_fn: Option<SetupFunc<T>>,
}

impl<T: Clone + Send + Sync + 'static> FrameworkBuilder<T> {
    pub fn new(
        registry: Arc<Registry<T>>,
        client: Client,
        app_id: Id<ApplicationMarker>,
        user_data: T,
    ) -> Self {
        Self {
            registry,
            client: Arc::new(client),
            app_id,
            user_data: Arc::new(user_data),
            setup_fn: None,
        }
    }

    pub fn setup(&mut self, func: SetupFunc<T>) -> &mut Self {
        self.setup_fn = Some(func);
        self
    }

    pub fn build(&self) -> Framework<T> {
        Framework::new(
            Arc::clone(&self.registry),
            Arc::clone(&self.client),
            self.app_id,
            Arc::clone(&self.user_data),
            self.setup_fn,
        )
    }
}
