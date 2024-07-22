use tracing::{debug, error, info, trace, warn};

#[derive(Default, Debug)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
    Fatal,
}

// SLoggerOptions are options when creating a new SLogger.
// A zero Options consists entirely of default values.
//
// SLoggerOptions are an extension of [slog.HandlerOptions].
#[derive(Default, Debug)]
pub struct SLogger {
    // Enable source code location (Default: false)
    pub add_source: bool,

    // Minimum level to log (Default: slog.LevelInfo)
    pub level: LogLevel,

    // ReplaceAttr is called to rewrite each non-group attribute before it is logged.
    // See https://pkg.go.dev/log/slog#HandlerOptions for details.
    //ReplaceAttr func(groups []string, attr slog.Attr) slog.Attr

    // Time format (Default: time.StampMilli)
    // only supported with text handler
    pub time_format: String,

    // Disable color (Default: false)
    // only supported with text handler (no color in json)
    pub no_color: bool,
}

impl SLogger {
    pub fn log(&self) {
        match self.level {
            LogLevel::Error => {
                error!("Error"); // TODO!
            }
            LogLevel::Warn => {
                warn!("Warn");
            }
            LogLevel::Info => {
                info!("Info");
            }
            LogLevel::Debug => {
                debug!("Debug");
            }
            LogLevel::Trace => {
                trace!("Trace");
            }
            LogLevel::Fatal => {
                error!("Fatal"); // TODO!
                                 // exit!!!
            }
        }
    }
}
