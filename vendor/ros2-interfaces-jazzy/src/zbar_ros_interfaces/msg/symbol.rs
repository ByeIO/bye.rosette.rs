use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    pub data: ::std::string::String,
    pub points: Vec<crate::vision_msgs::msg::Point2D>,
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol {
            data: ::std::string::String::new(),
            points: Vec::new(),
        }
    }
}

impl ros2_client::Message for Symbol {}
