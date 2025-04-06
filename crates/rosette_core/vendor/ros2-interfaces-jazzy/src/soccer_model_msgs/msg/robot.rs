use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Robot {
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
    pub twist: crate::geometry_msgs::msg::TwistWithCovariance,
    pub attributes: crate::soccer_vision_attribute_msgs::msg::Robot,
}

impl Default for Robot {
    fn default() -> Self {
        Robot {
            pose: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            twist: crate::geometry_msgs::msg::TwistWithCovariance::default(),
            attributes: crate::soccer_vision_attribute_msgs::msg::Robot::default(),
        }
    }
}

impl ros2_client::Message for Robot {}
