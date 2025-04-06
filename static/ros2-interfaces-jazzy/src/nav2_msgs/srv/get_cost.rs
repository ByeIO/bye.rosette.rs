use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostRequest {
    pub use_footprint: bool,
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Default for GetCostRequest {
    fn default() -> Self {
        GetCostRequest {
            use_footprint: false,
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }
}

impl ros2_client::Message for GetCostRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostResponse {
    pub cost: f32,
}

impl Default for GetCostResponse {
    fn default() -> Self {
        GetCostResponse {
            cost: 0.0,
        }
    }
}

impl ros2_client::Message for GetCostResponse {}


pub struct GetCost;
impl ros2_client::Service for GetCost {
    type Request = GetCostRequest;
    type Response = GetCostResponse;

    fn request_type_name(&self) -> &str { "GetCostRequest" }
    fn response_type_name(&self) -> &str { "GetCostResponse" }
}
