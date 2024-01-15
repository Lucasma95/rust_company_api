use crate::business::entities::country::Country;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CountryRepository: Send + Sync {
    async fn create(&self, country: Country) -> Result<Country, sqlx::Error>;
}

pub struct CountryRepositoryImpl {
    db: Pool<Postgres>,
}

impl CountryRepositoryImpl {
    pub fn new(db: Pool<Postgres>) -> Self {
        CountryRepositoryImpl { db: db }
    }
}

#[async_trait]
impl CountryRepository for CountryRepositoryImpl {
    async fn create(&self, country: Country) -> Result<Country, sqlx::Error> {
        let result = sqlx::query_as::<_, Country>(
            r#"
        INSERT INTO countries (name, continent)
        VALUES ($1, $2)
        RETURNING name, continent
        "#,
        )
        .bind(&country.name)
        .bind(&country.continent)
        .fetch_one(&self.db)
        .await;

        return result;
    }
}
