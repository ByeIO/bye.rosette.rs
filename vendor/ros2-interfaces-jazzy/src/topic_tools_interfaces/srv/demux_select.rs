use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxSelectRequest {
    pub topic: ::std::string::String,
}

impl Default for DemuxSelectRequest {
    fn default() -> Self {
        DemuxSelectRequest {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DemuxSelectRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxSelectResponse {
    pub prev_topic: ::std::string::String,
    pub success: bool,
}

impl Default for DemuxSelectResponse {
    fn default() -> Self {
        DemuxSelectResponse {
            prev_topic: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for DemuxSelectResponse {}


pub struct DemuxSelect;
impl ros2_client::Service for DemuxSelect {
    type Request = DemuxSelectRequest;
    type Response = DemuxSelectResponse;

    fn request_type_name(&self) -> &str { "DemuxSelectRequest" }
    fn response_type_name(&self) -> &str { "DemuxSelectResponse" }
}
