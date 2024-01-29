use std::sync::Arc;
use crate::repositories::company_repository::{CompanyDto, CompanyRepository};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GetCompaniesByCountry: Send + Sync {
    async fn get_by_country(&self, country_name: &str) -> Result<Vec<CompanyDto>, sqlx::Error>;
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
    async fn get_by_country(&self, country_name: &str) -> Result<Vec<CompanyDto>, sqlx::Error> {
        let companies = self.repository.get_by_country(country_name).await;
        match companies {
            Ok(companies) => Ok(companies),
            Err(err) => Err(err), //it's possible to generate a custom error from the sqlx one.
        }
    }
}
