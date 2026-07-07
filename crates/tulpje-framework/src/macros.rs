#[macro_export]
macro_rules! handler_func {
    ($func:expr $(,)?) => {
        |ctx| Box::pin($func(ctx))
    };
}

#[macro_export]
macro_rules! service_func {
    ($func:expr $(,)?) => {
        |ctx, shutdown_rx| Box::pin($func(ctx, shutdown_rx))
    };
}
