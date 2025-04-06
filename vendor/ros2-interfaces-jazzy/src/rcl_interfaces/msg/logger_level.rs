use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggerLevel {
    pub name: ::std::string::String,
    pub level: u32,
}

impl LoggerLevel {
    pub const LOG_LEVEL_UNKNOWN: u8 = 0;
    pub const LOG_LEVEL_DEBUG: u8 = 10;
    pub const LOG_LEVEL_INFO: u8 = 20;
    pub const LOG_LEVEL_WARN: u8 = 30;
    pub const LOG_LEVEL_ERROR: u8 = 40;
    pub const LOG_LEVEL_FATAL: u8 = 50;
}

impl Default for LoggerLevel {
    fn default() -> Self {
        LoggerLevel {
            name: ::std::string::String::new(),
            level: 0,
        }
    }
}

impl ros2_client::Message for LoggerLevel {}
