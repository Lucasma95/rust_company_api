use crate::business::use_cases::company::create_company::CreateCompany;
use crate::business::use_cases::company::get_companies_by_country::GetCompaniesByCountry;
use crate::business::use_cases::company::get_company_by_id::GetCompanyByID;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::http::contracts::company::CreateCompanyRequest;

#[post("/v1/company")]
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

#[get("/v1/company/{id}")]
pub async fn get_by_id(
    path: Path<uuid::Uuid>,
    usecase: Data<dyn GetCompanyByID>,
) -> impl Responder {
    let company_id = path.into_inner();

    let result = usecase.get_by_id(&company_id).await;

    match result {
        Ok(company) => return HttpResponse::Ok().json(json!(company)),
        Err(err) => {
            return HttpResponse::InternalServerError().json(err.to_string());
        }
    }
}

#[get("/v1/company/country/{country_id}")]
pub async fn get_by_country_id(
    path: Path<String>,
    usecase: Data<dyn GetCompaniesByCountry>,
) -> impl Responder {
    let company_name = path.into_inner();

    let result = usecase.get_by_country(&company_name).await;

    match result {
        Ok(company) => return HttpResponse::Ok().json(json!(company)),
        Err(err) => {
            return HttpResponse::InternalServerError().json(err.to_string());
        }
    }
}
