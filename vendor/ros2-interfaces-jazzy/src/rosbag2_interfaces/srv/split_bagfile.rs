use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitBagfileRequest {

}

impl Default for SplitBagfileRequest {
    fn default() -> Self {
        SplitBagfileRequest {

        }
    }
}

impl ros2_client::Message for SplitBagfileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitBagfileResponse {

}

impl Default for SplitBagfileResponse {
    fn default() -> Self {
        SplitBagfileResponse {

        }
    }
}

impl ros2_client::Message for SplitBagfileResponse {}


pub struct SplitBagfile;
impl ros2_client::Service for SplitBagfile {
    type Request = SplitBagfileRequest;
    type Response = SplitBagfileResponse;

    fn request_type_name(&self) -> &str { "SplitBagfileRequest" }
    fn response_type_name(&self) -> &str { "SplitBagfileResponse" }
}
