use std::time::Duration;
use std::{future::Future, pin::Pin, sync::Arc};

use tokio::{sync::mpsc, task::JoinHandle};
use tracing::{Instrument as _, Span};
use twilight_standby::Standby;

use crate::Metadata;
use crate::builder::FrameworkBuilder;
use crate::service_manager::ServiceManager;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use twilight_gateway::Event;
use twilight_http::Client;
use twilight_model::id::{Id, marker::ApplicationMarker};

use crate::handler::task_handler::TaskHandler;
use crate::scheduler::{SchedulerHandle, SchedulerTaskMessage};
use crate::{Context, Error, Registry};

pub(crate) type SetupFunc<T> =
    fn(ctx: Context<T>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
type EventMessage = (Metadata, Event, Option<Span>);

pub struct Framework<T: Clone + Send + Sync + 'static> {
    ctx: Context<T>,
    setup_fn: Option<SetupFunc<T>>,

    scheduler: SchedulerHandle<T>,
    dispatcher: DispatchHandle,
    service_manager: ServiceManager<T>,
}

impl<T: Clone + Send + Sync + 'static> Framework<T> {
    pub fn new(
        registry: Arc<Registry<T>>,
        client: Arc<Client>,
        application_id: Id<ApplicationMarker>,
        services: Arc<T>,
        setup_fn: Option<SetupFunc<T>>,
    ) -> Self {
        let ctx = Context {
            application_id,
            services,
            client,
            standby: Arc::new(Standby::new()),
        };
        let service_manager = ServiceManager::new(registry.services.clone());
        let scheduler =
            SchedulerHandle::new(registry.tasks.values().cloned().collect(), ctx.clone());
        let dispatcher = DispatchHandle::new(registry, ctx.clone());

        Self {
            ctx,
            setup_fn,

            scheduler,
            dispatcher,
            service_manager,
        }
    }

    pub fn builder(
        registry: Arc<Registry<T>>,
        client: Arc<Client>,
        application_id: Id<ApplicationMarker>,
        services: Arc<T>,
    ) -> FrameworkBuilder<T> {
        FrameworkBuilder::new(registry, client, application_id, services)
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        if let Some(setup_fn) = self.setup_fn.take() {
            (setup_fn)(self.ctx.clone())
                .await
                .map_err(|err| format!("error running setup function: {}", err))?;
        }

        self.scheduler
            .start()
            .map_err(|err| format!("error starting scheduled tasks: {}", err))?;

        self.service_manager
            .start(&self.ctx)
            .map_err(|err| format!("error starting services: {err}"))?;

        Ok(())
    }

    pub fn enable_task(
        &mut self,
        handler: TaskHandler<T>,
    ) -> Result<(), Box<mpsc::error::SendError<SchedulerTaskMessage<T>>>> {
        self.scheduler.enable_task(handler)
    }

    pub fn disable_task(
        &mut self,
        name: String,
    ) -> Result<(), Box<mpsc::error::SendError<SchedulerTaskMessage<T>>>> {
        self.scheduler.disable_task(name)
    }

    pub fn sender(&self) -> Sender {
        Sender {
            sender: self.dispatcher.sender.clone(),
        }
    }

    pub fn send(
        &mut self,
        meta: Metadata,
        event: Event,
        span: Option<Span>,
    ) -> Result<(), Box<mpsc::error::SendError<EventMessage>>> {
        self.dispatcher.send(meta, event, span)
    }

    pub async fn shutdown(&mut self) {
        self.scheduler.shutdown();
        self.dispatcher.shutdown();
        self.service_manager.shutdown();
    }

    pub async fn join(&mut self) -> Result<(), Error> {
        self.scheduler.join().await?;
        self.dispatcher.join().await?;
        self.service_manager.join().await?;

        Ok(())
    }
}

struct DispatchHandle {
    sender: mpsc::UnboundedSender<EventMessage>,
    shutdown: CancellationToken,
    handle: Option<JoinHandle<()>>,
}
impl DispatchHandle {
    fn new<T: Clone + Send + Sync + 'static>(registry: Arc<Registry<T>>, ctx: Context<T>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let shutdown = CancellationToken::new();

        let mut dispatch = Dispatch::new(ctx, registry, receiver, shutdown.child_token());
        let handle = Some(tokio::spawn(async move { dispatch.run().await }));

        Self {
            sender,
            shutdown,
            handle,
        }
    }

    fn send(
        &mut self,
        meta: Metadata,
        event: Event,
        span: Option<Span>,
    ) -> Result<(), Box<mpsc::error::SendError<EventMessage>>> {
        Ok(self.sender.send((meta, event, span))?)
    }

    fn shutdown(&mut self) {
        self.shutdown.cancel();
    }

    async fn join(&mut self) -> Result<(), Error> {
        Ok(self
            .handle
            .take()
            .ok_or("Dispatch already shutdown")?
            .await?)
    }
}

struct Dispatch<T: Clone + Send + Sync> {
    registry: Arc<Registry<T>>,
    ctx: Context<T>,

    receiver: mpsc::UnboundedReceiver<EventMessage>,
    shutdown: CancellationToken,

    tracker: TaskTracker,
}
impl<T: Clone + Send + Sync + 'static> Dispatch<T> {
    fn new(
        ctx: Context<T>,
        registry: Arc<Registry<T>>,

        receiver: mpsc::UnboundedReceiver<EventMessage>,
        shutdown: CancellationToken,
    ) -> Self {
        Self {
            registry,
            ctx,

            receiver,
            shutdown,

            tracker: TaskTracker::new(),
        }
    }

    async fn run(&mut self) {
        loop {
            tokio::select! {
                Some((meta, event, span)) = self.receiver.recv() => {
                    let registry = Arc::clone(&self.registry);
                    let ctx = self.ctx.clone();

                    self.tracker.spawn(async move {
                        crate::handle(meta, ctx, &registry, event).instrument(span.unwrap_or(Span::none())).await;
                    });
                },
                () = self.shutdown.cancelled() => break,
            }
        }

        self.receiver.close();
        self.tracker.close();

        if let Err(err) = tokio::time::timeout(Duration::from_secs(5), self.tracker.wait()).await {
            tracing::warn!("waiting for dispatch tasks timed out: {err}");
        };
    }
}

pub struct Sender {
    sender: mpsc::UnboundedSender<EventMessage>,
}

impl Sender {
    pub fn send(
        &self,
        meta: Metadata,
        event: Event,
    ) -> Result<(), Box<mpsc::error::SendError<EventMessage>>> {
        Ok(self.sender.send((meta, event, None))?)
    }

    pub fn with_span(
        &self,
        meta: Metadata,
        event: Event,
        span: Span,
    ) -> Result<(), Box<mpsc::error::SendError<EventMessage>>> {
        Ok(self.sender.send((meta, event, Some(span)))?)
    }

    pub fn closed(&self) -> bool {
        self.sender.is_closed()
    }
}
