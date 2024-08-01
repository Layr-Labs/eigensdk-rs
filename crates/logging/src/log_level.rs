#[derive(Default, Debug, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
    Fatal,
}
