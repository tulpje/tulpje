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
    enable_tasks: bool,
    enable_services: bool,
}

impl<T: Clone + Send + Sync + 'static> FrameworkBuilder<T> {
    pub fn new(
        registry: Arc<Registry<T>>,
        client: Arc<Client>,
        app_id: Id<ApplicationMarker>,
        user_data: Arc<T>,
    ) -> Self {
        Self {
            registry,
            client,
            app_id,
            user_data,

            setup_fn: None,
            enable_tasks: true,
            enable_services: true,
        }
    }

    pub fn setup(mut self, func: SetupFunc<T>) -> Self {
        self.setup_fn = Some(func);
        self
    }

    pub fn enable_tasks(mut self, val: bool) -> Self {
        self.enable_tasks = val;
        self
    }

    pub fn enable_services(mut self, val: bool) -> Self {
        self.enable_services = val;
        self
    }

    pub fn build(self) -> Framework<T> {
        Framework::new(
            self.registry,
            self.client,
            self.app_id,
            self.user_data,
            self.setup_fn,
            self.enable_tasks,
            self.enable_services,
        )
    }
}
