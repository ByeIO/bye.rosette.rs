use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadDockDatabaseRequest {
    pub filepath: ::std::string::String,
}

impl Default for ReloadDockDatabaseRequest {
    fn default() -> Self {
        ReloadDockDatabaseRequest {
            filepath: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReloadDockDatabaseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadDockDatabaseResponse {
    pub success: bool,
}

impl Default for ReloadDockDatabaseResponse {
    fn default() -> Self {
        ReloadDockDatabaseResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ReloadDockDatabaseResponse {}


pub struct ReloadDockDatabase;
impl ros2_client::Service for ReloadDockDatabase {
    type Request = ReloadDockDatabaseRequest;
    type Response = ReloadDockDatabaseResponse;

    fn request_type_name(&self) -> &str { "ReloadDockDatabaseRequest" }
    fn response_type_name(&self) -> &str { "ReloadDockDatabaseResponse" }
}
