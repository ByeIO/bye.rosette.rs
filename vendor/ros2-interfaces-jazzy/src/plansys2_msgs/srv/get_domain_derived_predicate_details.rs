use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainDerivedPredicateDetailsRequest {
    pub predicate: ::std::string::String,
}

impl Default for GetDomainDerivedPredicateDetailsRequest {
    fn default() -> Self {
        GetDomainDerivedPredicateDetailsRequest {
            predicate: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainDerivedPredicateDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainDerivedPredicateDetailsResponse {
    pub predicates: Vec<crate::plansys2_msgs::msg::Derived>,
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainDerivedPredicateDetailsResponse {
    fn default() -> Self {
        GetDomainDerivedPredicateDetailsResponse {
            predicates: Vec::new(),
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainDerivedPredicateDetailsResponse {}


pub struct GetDomainDerivedPredicateDetails;
impl ros2_client::Service for GetDomainDerivedPredicateDetails {
    type Request = GetDomainDerivedPredicateDetailsRequest;
    type Response = GetDomainDerivedPredicateDetailsResponse;

    fn request_type_name(&self) -> &str { "GetDomainDerivedPredicateDetailsRequest" }
    fn response_type_name(&self) -> &str { "GetDomainDerivedPredicateDetailsResponse" }
}
