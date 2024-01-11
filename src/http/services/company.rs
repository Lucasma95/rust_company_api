use actix_web::{ post, get, web::{Json, Data, Path}, HttpResponse, Responder};
use crate::business::use_cases::company::save_company::SaveCompany;
use crate::business::use_cases::company::get_company_by_id::GetCompanyByID;
use serde_json::json;

use crate::http::contracts::company::CreateCompanyRequest;

#[post("/company")]
pub async fn save(request: Json<CreateCompanyRequest>, usecase: Data<dyn SaveCompany>) -> impl Responder {
    let result = usecase.save(&request);

    match result {
        Ok(_) => {
            return HttpResponse::Ok().json(json!(request))
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!(request));
        }
    }
}

#[get("/company/{id}")]
pub async fn get_by_id(path: Path<uuid::Uuid>, usecase: Data<dyn GetCompanyByID>) -> impl Responder {

    let company_id = path.into_inner();

    let result = usecase.get_by_id(&company_id).await;

    match result {
        Ok(company) => {
            return HttpResponse::Ok().json(json!(company))
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json("error al querer obtener la empresa");

        }
    }
}

#[get("/v2/company/{id}")]
pub async fn get_by_id_v2(path: Path<uuid::Uuid>, usecase: Data<dyn GetCompanyByID>) -> impl Responder {

    let company_id = path.into_inner();

    let result = usecase.get_by_id(&company_id).await;

    match result {
        Ok(company) => {
            return HttpResponse::Ok().json(json!(company))
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json("error al querer obtener la empresa");

        }
    }
}
