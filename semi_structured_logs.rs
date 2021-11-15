/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message)
    }
}

pub fn info(message: &str) -> String {
    let mut str = String::from("[INFO]: ");
    str.push_str(message);

    str
}

pub fn warn(message: &str) -> String {
    let mut str = String::from("[WARNING]: ");
    str.push_str(message);

    str
}

pub fn error(message: &str) -> String {
    let mut str = String::from("[ERROR]: ");
    str.push_str(message);

    str
}
