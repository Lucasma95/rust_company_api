use crate::http::contracts::country::CreateCountryRequest;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub continent: String,
}

pub fn create_country_from_request(request: &CreateCountryRequest) -> Country {
    return Country {
        name: request.name.to_owned(),
        continent: request.continent.to_owned(),
    };
}
