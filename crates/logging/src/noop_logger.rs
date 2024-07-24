use super::logging::Logger;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct NoopLogger {}

impl Logger for NoopLogger {
    type LoggerType = NoopLogger;

    fn debug(&self, _msg: &str, _args: &[impl Debug]) {}
    fn info(&self, _msg: &str, _args: &[impl Debug]) {}
    fn warn(&self, _msg: &str, _args: &[impl Debug]) {}
    fn error(&self, _msg: &str, _args: &[impl Debug]) {}
    fn fatal(&self, _msg: &str, _args: &[impl Debug]) {}
}
