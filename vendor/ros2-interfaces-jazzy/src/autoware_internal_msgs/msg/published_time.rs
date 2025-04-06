use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishedTime {
    pub header: crate::std_msgs::msg::Header,
    pub published_stamp: crate::builtin_interfaces::msg::Time,
}

impl Default for PublishedTime {
    fn default() -> Self {
        PublishedTime {
            header: crate::std_msgs::msg::Header::default(),
            published_stamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for PublishedTime {}
