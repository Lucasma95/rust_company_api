pub mod http;
pub mod business;
pub mod repositories;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions};
use std::{env, sync::Arc};
use business::use_cases::company::save_company::SaveCompany;
use business::use_cases::company::get_company_by_id::GetCompanyByID;


#[get("/")]//probar pasarlo a /health
async fn hello() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    //env_logger::init();
    dotenv().ok();
    let port = define_port();

    print!("starting service at port: {}", port);
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        //.connect(&database_url)
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

    let company_repo = repositories::company_repository::CompanyRepositoryImpl::new(pool.clone());
    let company_repo_box = Box::new(company_repo);

    let save_company_usecase_impl = business::use_cases::company::save_company::SaveCompanyImpl::new();
    let save_usecase_arc = Arc::new(save_company_usecase_impl) as Arc<dyn SaveCompany>;

    let get_company_by_id_usecase_impl = business::use_cases::company::get_company_by_id::GetCompanyByIDImpl::new(company_repo_box);
    let get_company_by_id_usecase_arc = Arc::new(get_company_by_id_usecase_impl) as Arc<dyn GetCompanyByID>;

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(http::services::company::save)
            .service(http::services::company::get_by_id)
            .service(http::services::health::get_basic_health_status)
            .app_data(Data::from(save_usecase_arc.clone()))
            .app_data(Data::from(get_company_by_id_usecase_arc.clone()))
            //.route("/echo", web::post().to(http::services::post_message::echo))
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

/*fn define_port_long() -> u16 {
    let port = env::var("PORT");
    match port {
        Ok(port) => {
            let parsed_port: Result<u16, _> = port.parse();
            match parsed_port {
                Ok(value) => value,
                Err(_) => 8080,
            }
        }
        Err(_) => 8080,
    }
}*/
