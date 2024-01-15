use async_trait::async_trait;
use std::sync::Arc;

use crate::business::entities::country as entities;
use crate::http::contracts::country::CreateCountryRequest;
use crate::repositories::country_repository::CountryRepository;

#[async_trait]
pub trait CreateCountry: Send + Sync {
    async fn create(
        &self,
        request: &CreateCountryRequest,
    ) -> Result<entities::Country, sqlx::Error>;
}

pub struct CreateCountryImpl {
    repository: Arc<dyn CountryRepository>,
}

impl CreateCountryImpl {
    pub fn new(repository: Arc<dyn CountryRepository>) -> CreateCountryImpl {
        CreateCountryImpl {
            repository: repository,
        }
    }
}

#[async_trait]
impl CreateCountry for CreateCountryImpl {
    async fn create(
        &self,
        request: &CreateCountryRequest,
    ) -> Result<entities::Country, sqlx::Error> {
        let company_to_create = entities::create_country_from_request(request);
        let company = self.repository.create(company_to_create).await;
        match company {
            Ok(company) => Ok(company),
            Err(err) => Err(err),
        }
    }
}
