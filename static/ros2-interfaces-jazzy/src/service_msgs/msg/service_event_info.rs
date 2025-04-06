use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceEventInfo {
    pub event_type: u8,
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub client_gid: [i8; 16],
    pub sequence_number: i64,
}

impl ServiceEventInfo {
    pub const REQUEST_SENT: u8 = 0;
    pub const REQUEST_RECEIVED: u8 = 1;
    pub const RESPONSE_SENT: u8 = 2;
    pub const RESPONSE_RECEIVED: u8 = 3;
}

impl Default for ServiceEventInfo {
    fn default() -> Self {
        ServiceEventInfo {
            event_type: 0,
            stamp: crate::builtin_interfaces::msg::Time::default(),
            client_gid: [0; 16],
            sequence_number: 0,
        }
    }
}

impl ros2_client::Message for ServiceEventInfo {}
