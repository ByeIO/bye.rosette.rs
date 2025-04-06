use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseDeprecatedRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetPoseDeprecatedRequest {
    fn default() -> Self {
        SetPoseDeprecatedRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for SetPoseDeprecatedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseDeprecatedResponse {

}

impl Default for SetPoseDeprecatedResponse {
    fn default() -> Self {
        SetPoseDeprecatedResponse {

        }
    }
}

impl ros2_client::Message for SetPoseDeprecatedResponse {}


pub struct SetPoseDeprecated;
impl ros2_client::Service for SetPoseDeprecated {
    type Request = SetPoseDeprecatedRequest;
    type Response = SetPoseDeprecatedResponse;

    fn request_type_name(&self) -> &str { "SetPoseDeprecatedRequest" }
    fn response_type_name(&self) -> &str { "SetPoseDeprecatedResponse" }
}
