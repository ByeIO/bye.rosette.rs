use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTypeDescriptionRequest {
    pub type_name: ::std::string::String,
    pub type_hash: ::std::string::String,
    pub include_type_sources: bool, // default: true
}

impl Default for GetTypeDescriptionRequest {
    fn default() -> Self {
        GetTypeDescriptionRequest {
            type_name: ::std::string::String::new(),
            type_hash: ::std::string::String::new(),
            include_type_sources: true,
        }
    }
}

impl ros2_client::Message for GetTypeDescriptionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTypeDescriptionResponse {
    pub successful: bool,
    pub failure_reason: ::std::string::String,
    pub type_description: crate::type_description_interfaces::msg::TypeDescription,
    pub type_sources: Vec<crate::type_description_interfaces::msg::TypeSource>,
    pub extra_information: Vec<crate::type_description_interfaces::msg::KeyValue>,
}

impl Default for GetTypeDescriptionResponse {
    fn default() -> Self {
        GetTypeDescriptionResponse {
            successful: false,
            failure_reason: ::std::string::String::new(),
            type_description: crate::type_description_interfaces::msg::TypeDescription::default(),
            type_sources: Vec::new(),
            extra_information: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetTypeDescriptionResponse {}


pub struct GetTypeDescription;
impl ros2_client::Service for GetTypeDescription {
    type Request = GetTypeDescriptionRequest;
    type Response = GetTypeDescriptionResponse;

    fn request_type_name(&self) -> &str { "GetTypeDescriptionRequest" }
    fn response_type_name(&self) -> &str { "GetTypeDescriptionResponse" }
}
