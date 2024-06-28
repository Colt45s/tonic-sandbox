pub fn setup() {
    init();
}

#[cfg(not(debug_assertions))]
fn init() {
    tracing_subscriber::fmt()
        .json()
        .with_target(true)
        .with_ansi(false)
        .without_time()
        .init()
}

#[cfg(debug_assertions)]
fn init() {
    use tracing_subscriber::fmt::time::ChronoLocal;
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .compact()
        .with_target(true)
        .with_ansi(true)
        .with_timer(ChronoLocal::rfc_3339())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")))
        .init()
}
