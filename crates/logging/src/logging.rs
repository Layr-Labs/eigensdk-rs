use std::fmt::Debug;

pub trait Logger {
    type LoggerType: Logger;

    fn debug(&self, msg: &str, args: &[impl Debug]);
    fn info(&self, msg: &str, args: &[impl Debug]);
    fn warn(&self, msg: &str, args: &[impl Debug]);
    fn error(&self, msg: &str, args: &[impl Debug]);
    fn fatal(&self, msg: &str, args: &[impl Debug]);
}
