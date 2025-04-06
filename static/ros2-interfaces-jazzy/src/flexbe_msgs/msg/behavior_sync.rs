use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorSync {
    pub behavior_id: i32,
    pub current_state_checksums: Vec<i32>,
}

impl BehaviorSync {
    pub const INVALID: i32 = 0;
}

impl Default for BehaviorSync {
    fn default() -> Self {
        BehaviorSync {
            behavior_id: 0,
            current_state_checksums: Vec::new(),
        }
    }
}

impl ros2_client::Message for BehaviorSync {}
