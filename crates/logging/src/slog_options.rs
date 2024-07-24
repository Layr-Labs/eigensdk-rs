use super::logging::Logger;
use std::fmt::Arguments;
use std::fmt::Debug;
use std::process::exit;
use tracing::{debug, error, info, span, trace, warn, Event, Level, Metadata};

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
}

impl SLogger {
    pub fn new_full_logger(
        no_color: bool,
        time_format: String,
        level: LogLevel,
        add_source: bool,
    ) -> Self {
        tracing::subscriber::set_global_default(
            tracing_subscriber::fmt::Subscriber::builder()
                .with_max_level(tracing::Level::TRACE)
                .with_ansi(no_color)
                .finish(),
        )
        .expect("setting default subscriber failed");

        SLogger {
            add_source,
            level,
            time_format,
        }
    }

    pub fn new_compact_logger(
        no_color: bool,
        time_format: String,
        level: LogLevel,
        add_source: bool,
    ) -> Self {
        tracing::subscriber::set_global_default(
            tracing_subscriber::fmt::Subscriber::builder()
                .with_max_level(tracing::Level::TRACE)
                .with_ansi(no_color)
                .compact()
                .finish(),
        )
        .expect("setting default subscriber failed");

        SLogger {
            add_source,
            level,
            time_format,
        }
    }

    pub fn new_json_logger(
        no_color: bool,
        time_format: String,
        level: LogLevel,
        add_source: bool,
    ) -> Self {
        tracing::subscriber::set_global_default(
            tracing_subscriber::fmt::Subscriber::builder()
                .with_max_level(tracing::Level::TRACE)
                .with_ansi(no_color)
                .json()
                .finish(),
        )
        .expect("setting default subscriber failed");

        SLogger {
            add_source,
            level,
            time_format,
        }
    }

    pub fn log(&self, msg: &str, tags: &[impl Debug]) {
        match self.level {
            LogLevel::Error => {
                error!("Error");
                self.error(msg, tags);
            }
            LogLevel::Warn => {
                warn!("Warn");
                self.warn(msg, tags);
            }
            LogLevel::Info => {
                info!("Info");
                self.info(msg, tags);
            }
            LogLevel::Debug => {
                debug!("Debug");
                self.debug(msg, tags);
            }
            LogLevel::Trace => {
                trace!("Trace");
                self.debug(msg, tags);
            }
            LogLevel::Fatal => {
                error!("Fatal");
                self.error(msg, tags);
                exit(1); // exit process
            }
        }
    }
}

// NewSlogJSONLogger creates a new SLogger with a JSON handler
// Default behavior is colored log outputs. To disable colors, set opts.NoColor to true.

impl Logger for SLogger {
    type LoggerType = SLogger;

    fn debug(&self, msg: &str, tags: &[impl Debug]) {
        debug!("{} {:?}", msg, tags);
    }

    fn info(&self, msg: &str, tags: &[impl Debug]) {
        info!("{} {:?}", msg, tags);
    }

    fn warn(&self, msg: &str, tags: &[impl Debug]) {
        warn!("{} {:?}", msg, tags);
    }

    fn error(&self, msg: &str, tags: &[impl Debug]) {
        error!("{} {:?}", msg, tags);
    }

    fn fatal(&self, msg: &str, tags: &[impl Debug]) {
        error!("{} {:?}", msg, tags);
        exit(1); // exit process
    }
}
