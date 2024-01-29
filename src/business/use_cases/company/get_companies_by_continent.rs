use std::sync::Arc;
use crate::repositories::company_repository::{CompanyDto, CompanyRepository};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GetCompaniesByContinent: Send + Sync {
    async fn get_by_continent(&self, continent_name: &str) -> Result<Vec<CompanyDto>, sqlx::Error>;
}

pub struct GetCompaniesByContinentImpl {
    repository: Arc<dyn CompanyRepository>,
}

impl GetCompaniesByContinentImpl {
    pub fn new(repository: Arc<dyn CompanyRepository>) -> GetCompaniesByContinentImpl {
        GetCompaniesByContinentImpl {
            repository: repository,
        }
    }
}

#[async_trait]
impl GetCompaniesByContinent for GetCompaniesByContinentImpl {
    async fn get_by_continent(&self, continent_name: &str) -> Result<Vec<CompanyDto>, sqlx::Error> {
        let companies = self.repository.get_by_continent(continent_name).await;
        match companies {
            Ok(companies) => Ok(companies),
            Err(err) => Err(err), //it's possible to generate a custom error from the sqlx one.
        }
    }
}
