use crate::business::entities::company::Company;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Company, sqlx::Error>;
    async fn get_by_country(&self, country: String) -> Result<Vec<Company>, sqlx::Error>;
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
        return sqlx::query_as::<_, Company>(
            "Select id, name, description, country From companies WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.db)
        .await;
    }

    async fn get_by_country(&self, country: String) -> Result<Vec<Company>, sqlx::Error> {
        let query_result_2 = sqlx::query_as::<_, Company>(
            "Select id, name, description, country From companies WHERE country = $1",
        )
        .bind(country)
        .fetch_all(&self.db) //using fetch_many returns a stream in order to process high amount of entities
        .await;

        return query_result_2;
    }

    async fn create(&self, company: Company) -> Result<Company, sqlx::Error> {
        let result = sqlx::query_as::<_, Company>(
            r#"
        INSERT INTO companies (id, name, description, country)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, description, country
        "#,
        )
        .bind(&company.id)
        .bind(&company.name)
        .bind(&company.description)
        .bind(&company.country)
        .fetch_one(&self.db)
        .await;

        return result;
    }
}
