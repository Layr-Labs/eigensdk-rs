use super::log_level::LogLevel;
use super::logger::Logger;
use std::{fmt::Debug, sync::Arc};
use tracing::{debug, error, info, trace, warn};

// SLoggerOptions are options when creating a new SLogger.
// A zero Options consists entirely of default values.
//
// SLoggerOptions are an extension of [slog.HandlerOptions].
#[derive(Default, Debug, Clone)]
pub struct TracingLogger {
    // Enable source code location (Default: false)
    pub add_source: bool,

    // Minimum level to log (Default: slog.LevelInfo)
    pub level: LogLevel,

    // Time format (Default: time.StampMilli)
    // only supported with text handler
    pub time_format: String,
}

impl TracingLogger {
    pub fn new_text_logger(
        no_color: bool,
        time_format: String,
        level: LogLevel,
        add_source: bool,
    ) -> Arc<dyn Logger> {
        let tracing_level = match level {
            LogLevel::Fatal => tracing::Level::ERROR,
            LogLevel::Error => tracing::Level::ERROR,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Trace => tracing::Level::TRACE,
        };
        tracing::subscriber::set_global_default(
            tracing_subscriber::fmt::Subscriber::builder()
                .with_max_level(tracing_level)
                .with_ansi(no_color)
                .finish(),
        )
        .expect("setting default subscriber failed");

        Arc::new(TracingLogger {
            add_source,
            level,
            time_format,
        })
    }
}

pub fn create_tracing_logger(
    no_color: bool,
    time_format: String,
    level: LogLevel,
    add_source: bool,
) -> Arc<dyn Logger> {
    TracingLogger::new_text_logger(no_color, time_format, level, add_source)
}

impl Logger for TracingLogger {
    // type LoggerType = TracingLogger;

    fn debug(&self, msg: &str, tags: &str) {
        debug!("{} {:?}", msg, [tags]);
    }

    fn info(&self, msg: &str, tags: &str) {
        info!("{} {:?}", msg, [tags]);
    }

    fn warn(&self, msg: &str, tags: &str) {
        warn!("{} {:?}", msg, [tags]);
    }

    fn error(&self, msg: &str, tags: &str) {
        error!("{} {:?}", msg, [tags]);
    }

    fn fatal(&self, msg: &str, tags: &str) {
        error!("{} {:?}", msg, [tags]);
        panic!("Fatal error occurred: {} {:?}", msg, [tags]);
    }

    fn log(&self, msg: &str, tags: &str) {
        match self.level {
            LogLevel::Fatal => {
                error!("Fatal");
                self.fatal(msg, tags);
            }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_test_logger;

    #[test]
    fn test_log() {
        let logger = get_test_logger();
        logger.log("Log", "info logged");
    }
}
