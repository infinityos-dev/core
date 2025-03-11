use crate::print;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    None = 0,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Panic,
}

pub fn log(level: LogLevel, message: &str) {
    let (label, color_code) = match level {
        LogLevel::None => ("NONE ", "\x1b[90m"), // Bright Black (Gray), if needed
        LogLevel::Trace => ("TRACE", "\x1b[95m"), // Bright Magenta
        LogLevel::Debug => ("DEBUG", "\x1b[94m"), // Bright Blue
        LogLevel::Info => ("INFO ", "\x1b[92m"), // Bright Green
        LogLevel::Warn => ("WARN ", "\x1b[93m"), // Bright Yellow
        LogLevel::Error => ("ERROR", "\x1b[91m"), // Bright Red
        LogLevel::Fatal => ("FATAL", "\x1b[91m"), // Bright Red (same as ERROR)
        LogLevel::Panic => ("PANIC", "\x1b[97;41m"), // White text on Red background
    };

    print!("{}[{}]\x1b[0m {}", color_code, label, message);
}

pub fn trace(message: &str) {
    log(LogLevel::Trace, message);
}

pub fn debug(message: &str) {
    log(LogLevel::Debug, message);
}

pub fn info(message: &str) {
    log(LogLevel::Info, message);
}

pub fn warn(message: &str) {
    log(LogLevel::Warn, message);
}

pub fn error(message: &str) {
    log(LogLevel::Error, message);
}

pub fn fatal(message: &str) {
    log(LogLevel::Fatal, message);
}

pub fn panic(message: &str) {
    log(LogLevel::Panic, message);
}
