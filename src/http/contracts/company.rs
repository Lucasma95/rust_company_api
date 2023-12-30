use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateCompanyRequest {
    pub name: String,
    pub description: String,
    pub country: String,
}