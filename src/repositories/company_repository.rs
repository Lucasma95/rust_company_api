use crate::business::entities::company::Company;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Company, sqlx::Error>;
    async fn get_by_country(&self, country_name: &str) -> Result<Vec<CompanyDto>, sqlx::Error>;
    async fn get_by_continent(&self, continent_name: &str) -> Result<Vec<CompanyDto>, sqlx::Error>;
    async fn create(&self, company: Company) -> Result<Company, sqlx::Error>;
}

pub struct CompanyRepositoryImpl {
    db: Pool<Postgres>,
}

impl CompanyRepositoryImpl {
    pub fn new(db: Pool<Postgres>) -> Self {
        CompanyRepositoryImpl { db: db }
    }
}

#[async_trait]
impl CompanyRepository for CompanyRepositoryImpl {
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Company, sqlx::Error> {
        let result = sqlx::query_as::<_, Company>(
            r#"
            Select id, name, description, country_name, created_at, updated_at 
            From companies c 
            WHERE c.id = $1 AND c.deleted_at IS NULL"#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await;

        if result.is_err() {
            eprintln!("Error getting company by id: {}", id);
        }
        return result;
    }

    async fn create(&self, mut company: Company) -> Result<Company, sqlx::Error> {
        company.before_create();
        let result = sqlx::query_as::<_, Company>(
            r#"
            INSERT INTO companies (id, name, description, country_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, %6)
            RETURNING id, name, description, country_id"#,
        )
        .bind(&company.id)
        .bind(&company.name)
        .bind(&company.description)
        .bind(&company.country_name)
        .bind(&company.created_at.unwrap())
        .bind(&company.updated_at.unwrap())
        .fetch_one(&self.db)
        .await;

        if result.is_err() {
            eprintln!("Error creating company. [company_name]: {}", company.name);
        }
        return result;
    }

    async fn get_by_country(&self, country_name: &str) -> Result<Vec<CompanyDto>, sqlx::Error> {
        let result = sqlx::query_as::<_, CompanyDto>(
            r#"
            Select id, c.name, description, country_name, co.continent as continent_name  
            FROM companies c 
            JOIN countries co ON c.country_name = co.name 
            WHERE c.country_name = $1 AND c.deleted_at IS NULL"#,
        )
        .bind(country_name)
        .fetch_all(&self.db)
        .await;

        if result.is_err() {
            eprintln!("Error getting companies by country: {}", country_name)
        }
        return result;
    }

    async fn get_by_continent(&self, continent_name: &str) -> Result<Vec<CompanyDto>, sqlx::Error> {
        let result = sqlx::query_as::<_, CompanyDto>(
            r#"
            Select id, c.name, description, country_name, co.continent as continent_name 
            FROM companies c 
            JOIN countries co ON c.country_name = co.name 
            WHERE co.continent = $1 AND c.deleted_at IS NULL"#,
        )
        .bind(continent_name)
        .fetch_all(&self.db)
        .await;

        if result.is_err() {
            eprintln!("Error getting companies by continent: {}", continent_name)
        }
        return result;
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CompanyDto {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    #[sqlx(default)]
    #[serde(skip)]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    #[serde(skip)]
    pub updated_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    #[serde(skip)]
    pub deleted_at: Option<DateTime<Utc>>,
    pub country_name: String,
    pub continent_name: String,
}
