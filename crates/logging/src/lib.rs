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
use noop_logger::NoopLogger;
use once_cell::sync::OnceCell;
use tracing_logger::TracingLogger;

static TEST_LOGGER: OnceCell<EigenLogger> = OnceCell::new();

static LOGGER: OnceCell<EigenLogger> = OnceCell::new();

#[derive(Debug, Clone, Default)]
pub struct EigenLogger {
    pub noop_logger: Option<NoopLogger>,

    pub tracing_logger: Option<TracingLogger>,
}

impl EigenLogger {
    pub fn new_noop_logger(noop_logger: NoopLogger) -> Self {
        EigenLogger {
            noop_logger: Some(noop_logger),
            tracing_logger: None,
        }
    }

    pub fn new_tracing_logger(tracing_logger: TracingLogger) -> Self {
        EigenLogger {
            noop_logger: None,
            tracing_logger: Some(tracing_logger),
        }
    }
}

/// Initializes the test logger at the start using ctor.
#[ctor]
fn init_test_logger() {
    TEST_LOGGER.get_or_init(|| {
        EigenLogger::new_noop_logger(NoopLogger::new_text_logger(
            false,
            String::from(""),
            LogLevel::Debug,
            false,
        ))
    });
}

/// get the initialized test logger
pub fn get_test_logger() -> EigenLogger {
    EigenLogger::new_noop_logger(
        TEST_LOGGER
            .get()
            .expect("Logger not initialized")
            .noop_logger
            .clone()
            .unwrap(),
    )
}

/// Use this to initialize the tracer. It can only be initialized once.
/// Set the tracing level according to [`LogLevel`]
pub fn init_logger(log_level: LogLevel) {
    LOGGER.get_or_init(|| {
        EigenLogger::new_tracing_logger(TracingLogger::new_text_logger(
            false,
            String::from(""),
            log_level,
            false,
        ))
    });
}

/// get the initialized logger
pub fn get_logger() -> EigenLogger {
    EigenLogger::new_tracing_logger(
        LOGGER
            .get()
            .expect("Logger not initialized")
            .tracing_logger
            .clone()
            .unwrap(),
    )
}
