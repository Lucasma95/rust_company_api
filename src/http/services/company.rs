use crate::business::use_cases::company::get_companies_by_country::GetCompaniesByCountry;
use crate::business::use_cases::company::get_company_by_id::GetCompanyByID;
use crate::business::use_cases::company::{
    create_company::CreateCompany, get_companies_by_continent::GetCompaniesByContinent,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::http::contracts::company::CreateCompanyRequest;

#[post("/api/v1/company")]
pub async fn create(
    request: Json<CreateCompanyRequest>,
    usecase: Data<dyn CreateCompany>,
) -> impl Responder {
    let result = usecase.create(&request).await;

    match result {
        Ok(company) => return HttpResponse::Ok().json(json!(company)),
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!(err.to_string()));
        }
    }
}

#[get("/api/v1/company/{id}")]
pub async fn get_by_id(
    path: Path<uuid::Uuid>,
    usecase: Data<dyn GetCompanyByID>,
) -> impl Responder {
    let company_id = path.into_inner();

    let result = usecase.get_by_id(&company_id).await;

    match result {
        Ok(company) => return HttpResponse::Ok().json(json!(company)),
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(format!("No company found by id: {}", company_id))
        }
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/api/v1/company/country/{country_name}")]
pub async fn get_by_country_name(
    path: Path<String>,
    usecase: Data<dyn GetCompaniesByCountry>,
) -> impl Responder {
    let country_name = path.into_inner();

    let result = usecase.get_by_country(&country_name).await;

    match result {
        Ok(company) => return HttpResponse::Ok().json(json!(company)),
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/api/v1/company/continent/{continent_name}")]
pub async fn get_by_continent(
    path: Path<String>,
    usecase: Data<dyn GetCompaniesByContinent>,
) -> impl Responder {
    let continent_name = path.into_inner();

    let result = usecase.get_by_continent(&continent_name).await;

    match result {
        Ok(company) => return HttpResponse::Ok().json(json!(company)),
        Err(err) => {
            return HttpResponse::InternalServerError().json(err.to_string());
        }
    }
}
