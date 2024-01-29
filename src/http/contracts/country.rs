use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CreateCountryRequest {
    pub name: String,
    pub continent: String,
}