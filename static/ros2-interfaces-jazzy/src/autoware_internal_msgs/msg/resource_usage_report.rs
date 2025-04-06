use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceUsageReport {
    pub header: crate::std_msgs::msg::Header,
    pub pid: u32,
    pub cpu_cores_utilized: f32,
    pub total_memory_bytes: u64,
    pub free_memory_bytes: u64,
    pub process_memory_bytes: u64,
}

impl Default for ResourceUsageReport {
    fn default() -> Self {
        ResourceUsageReport {
            header: crate::std_msgs::msg::Header::default(),
            pid: 0,
            cpu_cores_utilized: 0.0,
            total_memory_bytes: 0,
            free_memory_bytes: 0,
            process_memory_bytes: 0,
        }
    }
}

impl ros2_client::Message for ResourceUsageReport {}
