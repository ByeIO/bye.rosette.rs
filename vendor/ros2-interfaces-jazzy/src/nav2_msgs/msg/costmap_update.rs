use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostmapUpdate {
    pub header: crate::std_msgs::msg::Header,
    pub x: u32,
    pub y: u32,
    pub size_x: u32,
    pub size_y: u32,
    pub data: Vec<u8>,
}

impl Default for CostmapUpdate {
    fn default() -> Self {
        CostmapUpdate {
            header: crate::std_msgs::msg::Header::default(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for CostmapUpdate {}
