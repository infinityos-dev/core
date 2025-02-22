use super::super::super::log::{log, LogLevel};

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
