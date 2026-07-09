use tracing_subscriber::EnvFilter;

/// Initializes the global tracing subscriber. Respects RUST_LOG env var,
/// defaults to "info" if not set.
pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(true)
        .with_thread_ids(false)
        .init();
}
