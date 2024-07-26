use super::log_level::LogLevel;
use super::logger::Logger;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct NoopLogger {}

impl Logger for NoopLogger {
    type LoggerType = NoopLogger;

    fn new_text_logger(
        _no_color: bool,
        _time_format: String,
        _level: LogLevel,
        _add_source: bool,
    ) -> Self::LoggerType {
        NoopLogger {}
    }

    fn new_json_logger(
        _no_color: bool,
        _time_format: String,
        _level: LogLevel,
        _add_source: bool,
    ) -> Self::LoggerType {
        NoopLogger {}
    }

    fn debug(&self, _msg: &str, _args: &[impl Debug]) {}
    fn info(&self, _msg: &str, _args: &[impl Debug]) {}
    fn warn(&self, _msg: &str, _args: &[impl Debug]) {}
    fn error(&self, _msg: &str, _args: &[impl Debug]) {}
    fn fatal(&self, _msg: &str, _args: &[impl Debug]) {}

    fn log(&self, _msg: &str, _args: &[impl Debug]) {}
}
