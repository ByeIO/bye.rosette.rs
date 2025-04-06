use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRequest {

}

impl Default for StopRequest {
    fn default() -> Self {
        StopRequest {

        }
    }
}

impl ros2_client::Message for StopRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopResponse {

}

impl Default for StopResponse {
    fn default() -> Self {
        StopResponse {

        }
    }
}

impl ros2_client::Message for StopResponse {}


pub struct Stop;
impl ros2_client::Service for Stop {
    type Request = StopRequest;
    type Response = StopResponse;

    fn request_type_name(&self) -> &str { "StopRequest" }
    fn response_type_name(&self) -> &str { "StopResponse" }
}
