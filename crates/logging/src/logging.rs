use std::fmt::Debug;

pub trait Logger {
    type LoggerType: Logger;

    fn debug(&self, msg: &str, tags: &[impl Debug]);
    fn info(&self, msg: &str, tags: &[impl Debug]);
    fn warn(&self, msg: &str, tags: &[impl Debug]);
    fn error(&self, msg: &str, tags: &[impl Debug]);
    fn fatal(&self, msg: &str, tags: &[impl Debug]);

    fn debugf(&self, template: &str, args: &[impl Debug]);
    fn infof(&self, template: &str, args: &[impl Debug]);
    fn warnf(&self, template: &str, args: &[impl Debug]);
    fn errorf(&self, template: &str, args: &[impl Debug]);
    fn fatalf(&self, template: &str, args: &[impl Debug]);

    fn with(&self, tags: &[impl Debug]) -> Box<Self::LoggerType>;
}
