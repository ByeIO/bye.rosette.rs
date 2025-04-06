use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoggerLevelsResult {
    pub successful: bool,
    pub reason: ::std::string::String,
}

impl Default for SetLoggerLevelsResult {
    fn default() -> Self {
        SetLoggerLevelsResult {
            successful: false,
            reason: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetLoggerLevelsResult {}
