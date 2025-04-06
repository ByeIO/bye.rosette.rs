use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoggerLevelsRequest {
    pub names: Vec<::std::string::String>,
}

impl Default for GetLoggerLevelsRequest {
    fn default() -> Self {
        GetLoggerLevelsRequest {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetLoggerLevelsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoggerLevelsResponse {
    pub levels: Vec<crate::rcl_interfaces::msg::LoggerLevel>,
}

impl Default for GetLoggerLevelsResponse {
    fn default() -> Self {
        GetLoggerLevelsResponse {
            levels: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetLoggerLevelsResponse {}


pub struct GetLoggerLevels;
impl ros2_client::Service for GetLoggerLevels {
    type Request = GetLoggerLevelsRequest;
    type Response = GetLoggerLevelsResponse;

    fn request_type_name(&self) -> &str { "GetLoggerLevelsRequest" }
    fn response_type_name(&self) -> &str { "GetLoggerLevelsResponse" }
}
