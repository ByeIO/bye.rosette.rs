use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: crate::type_description_interfaces::msg::FieldType,
    pub default_value: ::std::string::String,
}

impl Default for Field {
    fn default() -> Self {
        Field {
            name: ::std::string::String::new(),
            type_: crate::type_description_interfaces::msg::FieldType::default(),
            default_value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Field {}
