use crate::homedir::homedir;
use chrono::Utc;
use color_eyre::owo_colors::OwoColorize;
use std::fs::OpenOptions;
use std::io::Write;

pub enum Level {
    Debug,
    Info,
    Trace,
    Warn,
}

pub struct Logger {
    pub level: Level,
}

impl Logger {
    pub fn new(default_level: Level) -> Logger {
        Logger {
            level: default_level,
        }
    }

    fn default_log_file(&self) -> String {
        homedir() + "/.burst.log"
    }

    /// Info Log function
    /// Prints: [INFO] - message to the log file and stdout
    /// This the default Log Level for the Logger
    pub fn info(&self, message: &str) {
        self.log(Level::Info, message);
    }

    /// Debug Log function
    /// Prints: [DEBUG] - message to the log file and stdout
    /// This the debug Log Level for the Logger
    pub fn debug(&self, message: &str) {
        self.log(Level::Debug, message);
    }

    /// Trace Log function
    /// Prints: [TRACE] - message to the log file and stdout
    /// This the trace Log Level for the Logger
    pub fn trace(&self, message: &str) {
        self.log(Level::Trace, message);
    }

    /// Warn Log function
    /// Prints: [WARN] - message to the log file and stdout
    /// This the warn Log Level for the Logger
    pub fn warn(&self, message: &str) {
        self.log(Level::Warn, message);
    }

    fn log(&self, level: Level, message: &str) {
        let _colored_level = match level {
            Level::Debug => "DEBUG".green().bold().to_string(),
            Level::Info => "INFO".blue().bold().to_string(),
            Level::Trace => "TRACE".yellow().bold().to_string(),
            Level::Warn => "WARN".red().bold().to_string(),
        };

        let _default_level = match level {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Trace => "TRACE",
            Level::Warn => "WARN",
        };

        let _current_time = format!("[{}]", Utc::now().format("%Y-%m-%d %H:%M:%S,%3f"));
        let log_message = format!("{} - {} - {}", _current_time, _colored_level, message);
        let log_file_message = format!("{} - {} - {}\n", _current_time, _default_level, message);
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.default_log_file())
            .unwrap();

        file.write_all(log_file_message.as_bytes()).unwrap();
        println!("{}", log_message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let logger = Logger::new(Level::Info);
        logger.info("Info Log");
        logger.debug("Debug Log");
        logger.trace("Trace Log");
        logger.warn("Warn Log");
    }

    #[test]
    fn test_default_log_file() {
        let logger = Logger::new(Level::Info);
        assert_eq!(logger.default_log_file(), homedir() + "/.burst.log");
    }

    #[test]
    fn test_log() {
        let logger = Logger::new(Level::Info);
        logger.log(Level::Info, "Info Log");
        logger.log(Level::Debug, "Debug Log");
        logger.log(Level::Trace, "Trace Log");
        logger.log(Level::Warn, "Warn Log");
    }

    #[test]
    fn test_info() {
        let logger = Logger::new(Level::Info);
        logger.info("Info Log");
    }

    #[test]
    fn test_debug() {
        let logger = Logger::new(Level::Info);
        logger.debug("Debug Log");
    }

    #[test]
    fn test_trace() {
        let logger = Logger::new(Level::Info);
        logger.trace("Trace Log");
    }

    #[test]
    fn test_warn() {
        let logger = Logger::new(Level::Info);
        logger.warn("Warn Log");
    }
}