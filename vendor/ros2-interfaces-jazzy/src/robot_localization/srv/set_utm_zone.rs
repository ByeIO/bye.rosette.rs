use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUTMZoneRequest {
    pub utm_zone: ::std::string::String,
}

impl Default for SetUTMZoneRequest {
    fn default() -> Self {
        SetUTMZoneRequest {
            utm_zone: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetUTMZoneRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUTMZoneResponse {

}

impl Default for SetUTMZoneResponse {
    fn default() -> Self {
        SetUTMZoneResponse {

        }
    }
}

impl ros2_client::Message for SetUTMZoneResponse {}


pub struct SetUTMZone;
impl ros2_client::Service for SetUTMZone {
    type Request = SetUTMZoneRequest;
    type Response = SetUTMZoneResponse;

    fn request_type_name(&self) -> &str { "SetUTMZoneRequest" }
    fn response_type_name(&self) -> &str { "SetUTMZoneResponse" }
}
