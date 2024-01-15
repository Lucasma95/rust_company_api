use std::sync::Arc;

use crate::business::entities::company::Company;
use crate::repositories::company_repository::CompanyRepository;
use async_trait::async_trait;

#[async_trait]
pub trait GetCompaniesByCountry: Send + Sync {
    async fn get_by_country(&self, country_name: &str) -> Result<Vec<Company>, sqlx::Error>;
}

pub struct GetCompaniesByCountryImpl {
    repository: Arc<dyn CompanyRepository>,
}

impl GetCompaniesByCountryImpl {
    pub fn new(repository: Arc<dyn CompanyRepository>) -> GetCompaniesByCountryImpl {
        GetCompaniesByCountryImpl {
            repository: repository,
        }
    }
}

#[async_trait]
impl GetCompaniesByCountry for GetCompaniesByCountryImpl {
    async fn get_by_country(&self, country_name: &str) -> Result<Vec<Company>, sqlx::Error> {
        let company = self.repository.get_by_country(country_name).await;
        match company {
            Ok(company) => Ok(company),
            Err(err) => Err(err), //it's possible to generate a custom error from the sqlx one.
        }
    }
}
