use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoggerLevelsRequest {
    pub levels: Vec<crate::rcl_interfaces::msg::LoggerLevel>,
}

impl Default for SetLoggerLevelsRequest {
    fn default() -> Self {
        SetLoggerLevelsRequest {
            levels: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetLoggerLevelsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoggerLevelsResponse {
    pub results: Vec<crate::rcl_interfaces::msg::SetLoggerLevelsResult>,
}

impl Default for SetLoggerLevelsResponse {
    fn default() -> Self {
        SetLoggerLevelsResponse {
            results: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetLoggerLevelsResponse {}


pub struct SetLoggerLevels;
impl ros2_client::Service for SetLoggerLevels {
    type Request = SetLoggerLevelsRequest;
    type Response = SetLoggerLevelsResponse;

    fn request_type_name(&self) -> &str { "SetLoggerLevelsRequest" }
    fn response_type_name(&self) -> &str { "SetLoggerLevelsResponse" }
}
