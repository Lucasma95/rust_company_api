use crate::business::use_cases::country::create_country::CreateCountry;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::http::contracts::country::CreateCountryRequest;

#[post("/v1/country")]
pub async fn create(
    request: Json<CreateCountryRequest>,
    usecase: Data<dyn CreateCountry>,
) -> impl Responder {
    let result = usecase.create(&request).await;

    match result {
        Ok(country) => return HttpResponse::Ok().json(json!(country)),
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!(err.to_string()));
        }
    }
}
