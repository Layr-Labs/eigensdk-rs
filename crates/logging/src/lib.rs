#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod log_level;
pub mod logger;
pub mod noop_logger;
pub mod tracing_logger;
pub static COMPONENT_KEY: &str = "component";
use ctor::ctor;
use log_level::LogLevel;
use logger::Logger;
use once_cell::sync::OnceCell;
use tracing_logger::TracingLogger;

static TEST_LOGGER: OnceCell<TracingLogger> = OnceCell::new();

static LOGGER: OnceCell<TracingLogger> = OnceCell::new();

/// Initializes the test logger at the start using ctor.
#[ctor]
fn init_test_logger() {
    TEST_LOGGER.get_or_init(|| {
        TracingLogger::new_text_logger(false, String::from(""), LogLevel::Debug, false)
    });
}

/// get the initialized test logger
pub fn get_test_logger() -> &'static TracingLogger {
    TEST_LOGGER.get().expect("Logger not initialized")
}

///  Use this to initialize the tracer. It can only be initialized once.
/// Set the tracing level according to [`LogLevel`]
pub fn init_logger(log_level: LogLevel) {
    LOGGER
        .get_or_init(|| TracingLogger::new_text_logger(false, String::from(""), log_level, false));
}

/// get the initialized logger
pub fn get_logger() -> &'static TracingLogger {
    TEST_LOGGER.get().expect("Logger not initialized")
}
