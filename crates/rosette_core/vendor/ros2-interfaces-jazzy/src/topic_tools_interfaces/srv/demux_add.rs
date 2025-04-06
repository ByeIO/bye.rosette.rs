use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxAddRequest {
    pub topic: ::std::string::String,
}

impl Default for DemuxAddRequest {
    fn default() -> Self {
        DemuxAddRequest {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DemuxAddRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxAddResponse {
    pub success: bool,
}

impl Default for DemuxAddResponse {
    fn default() -> Self {
        DemuxAddResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for DemuxAddResponse {}


pub struct DemuxAdd;
impl ros2_client::Service for DemuxAdd {
    type Request = DemuxAddRequest;
    type Response = DemuxAddResponse;

    fn request_type_name(&self) -> &str { "DemuxAddRequest" }
    fn response_type_name(&self) -> &str { "DemuxAddResponse" }
}
