use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorTreeStatusChange {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub node_name: ::std::string::String,
    pub uid: u16,
    pub previous_status: ::std::string::String,
    pub current_status: ::std::string::String,
}

impl Default for BehaviorTreeStatusChange {
    fn default() -> Self {
        BehaviorTreeStatusChange {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            node_name: ::std::string::String::new(),
            uid: 0,
            previous_status: ::std::string::String::new(),
            current_status: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for BehaviorTreeStatusChange {}
