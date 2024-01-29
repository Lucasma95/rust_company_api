#[cfg(test)]
mod create_country_test {
    use crate::business::entities::country::Country;
    use crate::business::use_cases::country::create_country::{self as usecase, CreateCountry};
    use crate::http::contracts::country::CreateCountryRequest;
    use crate::http::services::country::create;
    use actix_web::web::Data;
    use actix_web::{test, App};
    use mockall::predicate::*;
    use serde_json::json;
    use sqlx::error::Error as SqlxError;
    use std::sync::Arc;

    #[actix_web::test]
    async fn test_create_country() {
        let mut usecase = usecase::MockCreateCountry::new();

        usecase
            .expect_create()
            .with(eq(create_country_request_mock()))
            .times(1)
            .returning(|_x| {
                Ok(Country {
                    name: "Japon".to_string(),
                    continent: "Asia".to_string(),
                })
            });

        let create_country_usecase_arc = Arc::new(usecase) as Arc<dyn CreateCountry>;

        let mut app = test::init_service(
            App::new()
                .service(create)
                .app_data(Data::from(create_country_usecase_arc.clone())),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/api/v1/country")
            .set_json(&create_country_request_mock())
            .to_request();

        let response = test::call_service(&mut app, request).await;

        assert_eq!(response.status(), 200);

        let body_bytes = test::read_body(response).await;
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(parsed_json, json!({"name": "Japon", "continent": "Asia"}));
    }

    #[actix_web::test]
    async fn test_create_country_fails() {
        let mut usecase = usecase::MockCreateCountry::new();

        usecase
            .expect_create()
            .with(eq(create_country_request_mock()))
            .times(1)
            .returning(|_x| {
                Err(SqlxError::Io(std::io::Error::from(
                    std::io::ErrorKind::Other,
                )))
            });

        let create_country_usecase_arc = Arc::new(usecase) as Arc<dyn CreateCountry>;

        let mut app = test::init_service(
            App::new()
                .service(create)
                .app_data(Data::from(create_country_usecase_arc.clone())),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/api/v1/country")
            .set_json(&create_country_request_mock())
            .to_request();

        let response = test::call_service(&mut app, request).await;

        assert_eq!(response.status(), 500);

        let body_bytes = test::read_body(response).await;
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(parsed_json, json!("error communicating with database: other error"));
    }

    fn create_country_request_mock() -> CreateCountryRequest {
        return CreateCountryRequest {
            name: "Japon".to_string(),
            continent: "Asia".to_string(),
        };
    }
}
