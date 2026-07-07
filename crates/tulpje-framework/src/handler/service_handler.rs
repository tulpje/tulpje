use std::{future::Future, pin::Pin};
use tokio_util::sync::CancellationToken;

use crate::Error;
use crate::context::TaskContext;

pub(crate) type ServiceFunc<T> = fn(
    TaskContext<T>,
    CancellationToken,
) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
