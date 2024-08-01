#![doc(
    html_logo_url = "https://github.com/supernovahs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/supernovahs/eigen-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod log_level;
pub mod logger;
pub mod noop_logger;
pub mod tracing_logger;
use log_level::LogLevel;
use logger::Logger;
use once_cell::sync::OnceCell;
use tracing_logger::TracingLogger;
pub static COMPONENT_KEY: &str = "component";

static LOGGER: OnceCell<TracingLogger> = OnceCell::new();

pub fn init_logger() -> &'static TracingLogger {
    LOGGER.get_or_init(|| {
        TracingLogger::new_text_logger(false, String::from(""), LogLevel::Info, false)
    })
}
