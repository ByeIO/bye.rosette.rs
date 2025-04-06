use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGraphRequest {
    pub graph: crate::fuse_msgs::msg::SerializedGraph,
}

impl Default for SetGraphRequest {
    fn default() -> Self {
        SetGraphRequest {
            graph: crate::fuse_msgs::msg::SerializedGraph::default(),
        }
    }
}

impl ros2_client::Message for SetGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGraphResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetGraphResponse {
    fn default() -> Self {
        SetGraphResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetGraphResponse {}


pub struct SetGraph;
impl ros2_client::Service for SetGraph {
    type Request = SetGraphRequest;
    type Response = SetGraphResponse;

    fn request_type_name(&self) -> &str { "SetGraphRequest" }
    fn response_type_name(&self) -> &str { "SetGraphResponse" }
}
