use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxDeleteRequest {
    pub topic: ::std::string::String,
}

impl Default for DemuxDeleteRequest {
    fn default() -> Self {
        DemuxDeleteRequest {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DemuxDeleteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxDeleteResponse {
    pub success: bool,
}

impl Default for DemuxDeleteResponse {
    fn default() -> Self {
        DemuxDeleteResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for DemuxDeleteResponse {}


pub struct DemuxDelete;
impl ros2_client::Service for DemuxDelete {
    type Request = DemuxDeleteRequest;
    type Response = DemuxDeleteResponse;

    fn request_type_name(&self) -> &str { "DemuxDeleteRequest" }
    fn response_type_name(&self) -> &str { "DemuxDeleteResponse" }
}
