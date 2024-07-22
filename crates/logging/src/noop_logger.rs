use super::logging::Logger;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct NoopLogger {}

impl Logger for NoopLogger {
    type LoggerType = NoopLogger;

    fn debug(&self, _msg: &str, _tags: &[impl Debug]) {}
    fn info(&self, _msg: &str, _tags: &[impl Debug]) {}
    fn warn(&self, _msg: &str, _tags: &[impl Debug]) {}
    fn error(&self, _msg: &str, _tags: &[impl Debug]) {}
    fn fatal(&self, _msg: &str, _tags: &[impl Debug]) {}

    fn debugf(&self, _template: &str, _args: &[impl Debug]) {}
    fn infof(&self, _template: &str, _args: &[impl Debug]) {}
    fn warnf(&self, _template: &str, _args: &[impl Debug]) {}
    fn errorf(&self, _template: &str, _args: &[impl Debug]) {}
    fn fatalf(&self, _template: &str, _args: &[impl Debug]) {}

    fn with(&self, _tags: &[impl Debug]) -> Box<Self::LoggerType> {
        Box::new(NoopLogger {})
    }
}
