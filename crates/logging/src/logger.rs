use std::fmt::Debug;
use std::sync::Arc;

pub type SharedLogger = Arc<dyn Logger>;

pub fn tags_as_debug<'a>(tags: &'a [&'a str]) -> Vec<&'a dyn Debug> {
    tags.iter().map(|tag| tag as &dyn Debug).collect()
}

pub trait Logger: Debug + Send + Sync {
    fn debug(&self, msg: &str, args: &str);
    fn info(&self, msg: &str, args: &str);
    fn warn(&self, msg: &str, args: &str);
    fn error(&self, msg: &str, args: &str);
    fn fatal(&self, msg: &str, args: &str);
    fn log(&self, msg: &str, args: &str);
}
