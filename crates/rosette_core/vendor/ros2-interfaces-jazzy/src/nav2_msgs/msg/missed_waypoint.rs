use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissedWaypoint {
    pub index: u32,
    pub goal: crate::geometry_msgs::msg::PoseStamped,
    pub error_code: u16,
}

impl Default for MissedWaypoint {
    fn default() -> Self {
        MissedWaypoint {
            index: 0,
            goal: crate::geometry_msgs::msg::PoseStamped::default(),
            error_code: 0,
        }
    }
}

impl ros2_client::Message for MissedWaypoint {}
