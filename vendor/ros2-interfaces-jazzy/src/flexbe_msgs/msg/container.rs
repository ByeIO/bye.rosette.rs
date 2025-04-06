use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub state_id: i32,
    pub path: ::std::string::String,
    pub children: Vec<::std::string::String>,
    pub outcomes: Vec<::std::string::String>,
    pub transitions: Vec<::std::string::String>,
    #[serde(rename = "type")]    pub type_: i8,
    pub autonomy: Vec<i8>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            state_id: 0,
            path: ::std::string::String::new(),
            children: Vec::new(),
            outcomes: Vec::new(),
            transitions: Vec::new(),
            type_: 0,
            autonomy: Vec::new(),
        }
    }
}

impl ros2_client::Message for Container {}
