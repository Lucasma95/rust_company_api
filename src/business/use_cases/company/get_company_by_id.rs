use std::sync::Arc;

use crate::business::entities::company::Company;
use crate::repositories::company_repository::CompanyRepository;
use async_trait::async_trait;

#[async_trait]
pub trait GetCompanyByID: Send + Sync {
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Company, sqlx::Error>;
}

pub struct GetCompanyByIDImpl {
    repository: Arc<dyn CompanyRepository>,
}

impl GetCompanyByIDImpl {
    pub fn new(repository: Arc<dyn CompanyRepository>) -> GetCompanyByIDImpl {
        GetCompanyByIDImpl {
            repository: repository,
        }
    }
}

#[async_trait]
impl GetCompanyByID for GetCompanyByIDImpl {
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Company, sqlx::Error> {
        let company = self.repository.get_by_id(id).await;
        match company {
            Ok(company) => Ok(company),
            Err(err) => Err(err), //it's possible to generate a custom error from the sqlx one.
        }
    }
}
