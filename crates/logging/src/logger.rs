use super::log_level::LogLevel;
use std::fmt::Debug;

pub trait Logger {
    type LoggerType: Logger;

    fn new_text_logger(
        no_color: bool,
        time_format: String,
        level: LogLevel,
        add_source: bool,
    ) -> Self::LoggerType;

    fn new_json_logger(
        no_color: bool,
        time_format: String,
        level: LogLevel,
        add_source: bool,
    ) -> Self::LoggerType;

    fn debug(&self, msg: &str, args: &[impl Debug]);
    fn info(&self, msg: &str, args: &[impl Debug]);
    fn warn(&self, msg: &str, args: &[impl Debug]);
    fn error(&self, msg: &str, args: &[impl Debug]);
    fn fatal(&self, msg: &str, args: &[impl Debug]);

    fn log(&self, msg: &str, args: &[impl Debug]);
}
