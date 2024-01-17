pub mod business;
pub mod http;
pub mod repositories;
use actix_web::{web::Data, App, HttpServer};
use business::use_cases::company::create_company::CreateCompany;
use business::use_cases::company::get_companies_by_country::GetCompaniesByCountry;
use business::use_cases::company::get_company_by_id::GetCompanyByID;
use business::use_cases::country::create_country::CreateCountry;
use dotenvy::dotenv;
use repositories::company_repository::CompanyRepository;
use repositories::country_repository::CountryRepository;
use sqlx;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenv().ok();
    let port = define_port();

    print!("starting service at port: {}", port);

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    
    let migration_result = sqlx::migrate!().run(&pool).await;
    match migration_result {
        Ok(()) => {
            print!("migration finished successfully. ✅")
        }
        Err(error) => {
            panic!("migration failed. [error]: {}", error.to_string())
        }
    }

    //country infrastructure
    let country_repo = repositories::country_repository::CountryRepositoryImpl::new(pool.clone());
    let country_repo_arc = Arc::new(country_repo) as Arc<dyn CountryRepository>;

    let create_country_usecase_impl =
        business::use_cases::country::create_country::CreateCountryImpl::new(Arc::clone(
            &country_repo_arc,
        ));
    let create_country_usecase_arc =
        Arc::new(create_country_usecase_impl) as Arc<dyn CreateCountry>;

    //company infrastructure
    let company_repo = repositories::company_repository::CompanyRepositoryImpl::new(pool.clone());
    let company_repo_arc = Arc::new(company_repo) as Arc<dyn CompanyRepository>;

    let create_company_usecase_impl =
        business::use_cases::company::create_company::CreateCompanyImpl::new(Arc::clone(
            &company_repo_arc,
        ));
    let create_company_usecase_arc =
        Arc::new(create_company_usecase_impl) as Arc<dyn CreateCompany>;

    let get_company_by_id_usecase_impl =
        business::use_cases::company::get_company_by_id::GetCompanyByIDImpl::new(Arc::clone(
            &company_repo_arc,
        ));
    let get_company_by_id_usecase_arc =
        Arc::new(get_company_by_id_usecase_impl) as Arc<dyn GetCompanyByID>;

    let get_companies_by_country_usecase_impl =
        business::use_cases::company::get_companies_by_country::GetCompaniesByCountryImpl::new(
            Arc::clone(&company_repo_arc),
        );
    let get_companies_by_country_usecase_arc =
        Arc::new(get_companies_by_country_usecase_impl) as Arc<dyn GetCompaniesByCountry>;

    HttpServer::new(move || {
        App::new()
            .service(http::services::health::get_basic_health_status)
            .service(http::services::country::create)
            .service(http::services::company::create)
            .service(http::services::company::get_by_id)
            .service(http::services::company::get_by_country_id)
            .app_data(Data::from(create_country_usecase_arc.clone()))
            .app_data(Data::from(create_company_usecase_arc.clone()))
            .app_data(Data::from(get_company_by_id_usecase_arc.clone()))
            .app_data(Data::from(get_companies_by_country_usecase_arc.clone()))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn define_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8080)
}
