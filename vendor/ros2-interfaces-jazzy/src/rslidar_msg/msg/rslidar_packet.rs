use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RslidarPacket {
    pub header: crate::std_msgs::msg::Header,
    pub is_difop: u8,
    pub is_frame_begin: u8,
    pub data: Vec<u8>,
}

impl Default for RslidarPacket {
    fn default() -> Self {
        RslidarPacket {
            header: crate::std_msgs::msg::Header::default(),
            is_difop: 0,
            is_frame_begin: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for RslidarPacket {}
