use std::collections::HashMap;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::{Context, context::TaskContext, handler::service_handler::ServiceFunc};

pub(crate) struct ServiceManager<T: Clone + Send + Sync + 'static> {
    services: HashMap<String, ServiceFunc<T>>,
    shutdown: CancellationToken,
    handles: HashMap<String, JoinHandle<()>>,
}

impl<T: Clone + Send + Sync> ServiceManager<T> {
    pub(crate) fn new(services: HashMap<String, ServiceFunc<T>>) -> Self {
        let shutdown = CancellationToken::new();

        Self {
            services,
            shutdown,
            handles: HashMap::new(),
        }
    }

    pub(crate) fn start(&mut self, ctx: &Context<T>) -> Result<(), crate::Error> {
        for (name, func) in &self.services {
            tracing::info!("Starting service {name} ...");

            let token = self.shutdown.clone();
            let ctx = TaskContext::from_context(ctx.clone());
            let func = *func;
            let inner_name = name.clone();

            let handle = tokio::spawn(async move {
                if let Err(err) = func(ctx, token).await {
                    tracing::warn!("error starting service {inner_name}: {err}");
                }
            });

            self.handles.insert(name.clone(), handle);
        }
        Ok(())
    }

    pub(crate) fn shutdown(&mut self) {
        tracing::info!("shutting down service manager ...");
        self.shutdown.cancel();
    }

    pub(crate) async fn join(&mut self) -> Result<(), crate::Error> {
        for (name, handle) in self.handles.drain() {
            match handle.await {
                Ok(()) => tracing::info!("stopped service {name}"),
                Err(err) => tracing::warn!("error stopping service {name}: {err}"),
            }
        }

        Ok(())
    }
}
