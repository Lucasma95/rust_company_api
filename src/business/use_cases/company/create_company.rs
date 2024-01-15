use async_trait::async_trait;
use std::sync::Arc;

use crate::business::entities::company as entities;
use crate::http::contracts::company::CreateCompanyRequest;
use crate::repositories::company_repository::CompanyRepository;

#[async_trait]
pub trait CreateCompany: Send + Sync {
    async fn create(
        &self,
        request: &CreateCompanyRequest,
    ) -> Result<entities::Company, sqlx::Error>;
}

pub struct CreateCompanyImpl {
    repository: Arc<dyn CompanyRepository>,
}

impl CreateCompanyImpl {
    pub fn new(repository: Arc<dyn CompanyRepository>) -> CreateCompanyImpl {
        CreateCompanyImpl {
            repository: repository,
        }
    }
}

#[async_trait]
impl CreateCompany for CreateCompanyImpl {
    async fn create(
        &self,
        request: &CreateCompanyRequest,
    ) -> Result<entities::Company, sqlx::Error> {
        let company_to_create = entities::create_company_from_request(request);
        let company = self.repository.create(company_to_create).await;
        match company {
            Ok(company) => Ok(company),
            Err(err) => Err(err),
        }
    }
}
