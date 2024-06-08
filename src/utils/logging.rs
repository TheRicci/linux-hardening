use env_logger::Builder;
use log::LevelFilter;
use std::env;

pub fn init_logging() {
    let mut builder = Builder::new();

    // Set the default log level to info, can be overridden by RUST_LOG env variable
    builder.filter(None, LevelFilter::Info);

    // Check if RUST_LOG is set in the environment and use it if available
    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse_filters(&rust_log);
    }

    builder.init();
}
