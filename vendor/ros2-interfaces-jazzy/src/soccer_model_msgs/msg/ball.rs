use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ball {
    pub header: crate::std_msgs::msg::Header,
    pub point: crate::soccer_geometry_msgs::msg::PointWithCovariance,
    pub twist: crate::geometry_msgs::msg::TwistWithCovariance,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            header: crate::std_msgs::msg::Header::default(),
            point: crate::soccer_geometry_msgs::msg::PointWithCovariance::default(),
            twist: crate::geometry_msgs::msg::TwistWithCovariance::default(),
        }
    }
}

impl ros2_client::Message for Ball {}
