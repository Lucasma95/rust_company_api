use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CreateCompanyRequest {
    pub name: String,
    pub description: String,
    pub country_name: String,
}