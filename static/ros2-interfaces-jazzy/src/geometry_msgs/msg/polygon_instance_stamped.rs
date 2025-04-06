use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolygonInstanceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub polygon: crate::geometry_msgs::msg::PolygonInstance,
}

impl Default for PolygonInstanceStamped {
    fn default() -> Self {
        PolygonInstanceStamped {
            header: crate::std_msgs::msg::Header::default(),
            polygon: crate::geometry_msgs::msg::PolygonInstance::default(),
        }
    }
}

impl ros2_client::Message for PolygonInstanceStamped {}
