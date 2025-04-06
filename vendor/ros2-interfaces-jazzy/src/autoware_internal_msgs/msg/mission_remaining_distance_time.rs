use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionRemainingDistanceTime {
    pub remaining_distance: f64,
    pub remaining_time: f64,
}

impl Default for MissionRemainingDistanceTime {
    fn default() -> Self {
        MissionRemainingDistanceTime {
            remaining_distance: 0.0,
            remaining_time: 0.0,
        }
    }
}

impl ros2_client::Message for MissionRemainingDistanceTime {}
