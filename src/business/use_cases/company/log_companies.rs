use std::sync::Arc;

use crate::repositories::company_repository::CompanyRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait LogCompany: Send + Sync {
    async fn log_companies_by_country(&self, country_name: String);
}

pub struct LogCompanyImpl {
    repository: Arc<dyn CompanyRepository>,
}

impl LogCompanyImpl {
    pub fn new(repository: Arc<dyn CompanyRepository>) -> LogCompanyImpl {
        LogCompanyImpl {
            repository: repository,
        }
    }
}

#[async_trait]
impl LogCompany for LogCompanyImpl {
    async fn log_companies_by_country(&self, country_name: String) {
        let companies = self.repository.get_by_country(&country_name).await;

        match companies {
            Err(err) => print!("error getting companies by country_name: {}, error: {}", country_name, err.to_string()),
            Ok(companies) => {
                println!("----- loguin {} companies", companies.len());

                for company in companies {
                    println!("id: {}", company.id);
                    println!("name: {}", company.name);
                    println!("description: {}", company.description);
                    println!("country_name: {}", company.country_name);
                    println!("created_at: {}", get_date_as_string(company.created_at));
                    println!("updated_at: {}", get_date_as_string(company.updated_at));
                    println!("deleted_at: {}", get_date_as_string(company.deleted_at));
                }
            },
        }
    }
}

fn get_date_as_string(date:  Option<DateTime<Utc>>) -> String {
    match date {
        Some(date) => return date.to_rfc3339(),
        None => return String::from("no data"),
    };
}
