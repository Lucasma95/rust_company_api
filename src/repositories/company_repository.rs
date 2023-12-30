use crate::business::entities::company::Company;
//use sqlx::{Pool, Postgres};
use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn get_by_id(&self, id: String) ->  Result<Company, sqlx::Error>;
}
//fn get_by_country(company: &str) -> Vec<Company>;

pub struct CompanyRepositoryImpl {
    db: Pool<Postgres>,
}

impl CompanyRepositoryImpl {
    pub fn new(db: Pool<Postgres>) -> Self {
        CompanyRepositoryImpl { db: db }
    }
}

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Company2 {
    pub id: String,
    pub name: String,
    pub description: String,
    pub country: String,
}

#[async_trait]
impl CompanyRepository for CompanyRepositoryImpl {
    async fn get_by_id(&self, id: String) ->  Result<Company, sqlx::Error> {


    //let query_result = sqlx::query_as!(Company, "Select id, name, description, country From companies WHERE id = $1", id)
    //.fetch_one(&self.db)
    //.await;

    //return  query_result;

    let query_result_2 = sqlx::query_as::<_, Company>("Select id, name, description, country From companies WHERE id = $1").bind(id).fetch_one(&self.db).await;

    let row = sqlx::query("Select id, name, description, country FROM companies WHERE id = '1'")
        .fetch_one(&self.db)
        .await?;

    // Obtener valores de las columnas
    let value1: String = row.try_get("id")?;
    let value2: String = row.try_get("name")?;

    // Hacer algo con los valores obtenidos
    println!("Column1: {}, Column2: {}", value1, value2);

    return query_result_2;

    //return Ok(Company { id: id, name: "name".to_string(), description: "description".to_string(), country: "country".to_string() });

        //let query_result = sqlx::query_as!(Company2,"Select id, name, description, country From companies WHERE id = $1",id).fetch

        //https://github.com/wpcodevo/rust-postgres-crud-sqlx/blob/master/src/handler.rs chatgpt

        //return Ok(Company { id: id, name: "name".to_string(), description: "description".to_string(), country: "country".to_string() })


        /*dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");


        match sqlx::query_as::<_, Company>("SELECT id, first_name, last_name FROM users")
        .fetch_all(pool)
        .await
        {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users found"),
        }*/
    }
    
    //fn save(company: Company) -> Result<(), diesel::result::Error> {
    //}

    //fn get_by_country(company: &str) -> Vec<Company> {
        //vec![] }
}
