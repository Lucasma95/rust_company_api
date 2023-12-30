use actix_web::{ get, HttpResponse, Responder};


#[get("/health")]
pub async fn get_basic_health_status() -> impl Responder {
    return HttpResponse::Ok().json("healthy");
}
