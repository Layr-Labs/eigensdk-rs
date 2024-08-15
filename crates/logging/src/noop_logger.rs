use super::log_level::LogLevel;
use super::logger::Logger;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct NoopLogger {}

impl NoopLogger {
    pub fn new_text_logger(
        _no_color: bool,
        _time_format: String,
        _level: LogLevel,
        _add_source: bool,
    ) -> Arc<dyn Logger> {
        Arc::new(NoopLogger {})
    }

    pub fn new_json_logger(
        _no_color: bool,
        _time_format: String,
        _level: LogLevel,
        _add_source: bool,
    ) -> Self {
        NoopLogger {}
    }
}

impl Logger for NoopLogger {
    fn debug(&self, _msg: &str, _args: &str) {}
    fn info(&self, _msg: &str, _args: &str) {}
    fn warn(&self, _msg: &str, _args: &str) {}
    fn error(&self, _msg: &str, _args: &str) {}
    fn fatal(&self, _msg: &str, _args: &str) {}

    fn log(&self, _msg: &str, _args: &str) {}
}
