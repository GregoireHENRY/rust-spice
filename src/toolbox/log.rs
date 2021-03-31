use simplelog::*;
use std::fs::File;

/// Location of the log file.
const LOG_FILE: &str = "rsc/log/main.log";
/// Default filter for the terminal logging.
pub const DEFAULT_LEVEL_TERM: LevelFilter = LevelFilter::Warn;
/// Default filter for the file logging.
pub const DEFAULT_LEVEL_WRITE: LevelFilter = LevelFilter::Info;

/// Alias for filters.
pub type LevelFilter = simplelog::LevelFilter;

/// Function to initialize the logs.
pub fn log_init(term_level: LevelFilter, write_level: LevelFilter) {
    CombinedLogger::init(vec![
        TermLogger::new(term_level, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            write_level,
            Config::default(),
            File::create(LOG_FILE).unwrap(),
        ),
    ])
    .unwrap();
}

/// Macro for the function to initialize the logs for default usage.
#[macro_export]
macro_rules! log_init {
    () => {
        $crate::toolbox::log::log_init(
            $crate::toolbox::log::DEFAULT_LEVEL_TERM,
            $crate::toolbox::log::DEFAULT_LEVEL_WRITE,
        )
    };
    ($term_level:expr) => {
        $crate::toolbox::log::log_init($term_level, $crate::toolbox::log::DEFAULT_LEVEL_WRITE)
    };
    ($term_level:expr, $write_level:expr) => {
        $crate::toolbox::log::log_init($term_level, $write_level)
    };
}

/// Alias for error logs.
#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        log::error!(target: $target, $($arg)+)
    );
    ($($arg:tt)+) => (
        log::error!($($arg)+)
    )
}

/// Alias for warn logs.
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        log::warn!(target: $target, $($arg)+)
    );
    ($($arg:tt)+) => (
        log::warn!($($arg)+)
    )
}

/// Alias for info logs.
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        log::info!(target: $target, $($arg)+)
    );
    ($($arg:tt)+) => (
        log::info!($($arg)+)
    )
}

/// Alias for debug logs.
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {
        log::debug!(target: $target, $($arg)+)
    };
    ($($arg:tt)+) => {
        log::debug!($($arg)+)
    };
    () => {
        log::debug!("")
    }
}

/// Alias for trace logs.
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {
        log::trace!(target: $target, $($arg)+)
    };
    ($($arg:tt)+) => {
        log::trace!($($arg)+)
    };
    () => {
        log::trace!("")
    }
}
