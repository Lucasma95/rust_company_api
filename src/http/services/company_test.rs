#[cfg(test)]
mod company_service_test {

    mod create_company_service_test {

        use crate::business::entities::company::create_company_from_request;
        use crate::business::use_cases::company::create_company;
        use crate::business::use_cases::company::create_company::CreateCompany;
        use crate::http::contracts::company::CreateCompanyRequest;
        use crate::http::services::company::create;
        use actix_web::web::Data;
        use actix_web::{test, App};
        use mockall::predicate::*;
        use serde_json::json;
        use serde_json::Value;
        use sqlx::error::Error as SqlxError;
        use std::sync::Arc;

        #[actix_web::test]
        async fn test_company_successfully() {
            let mut usecase = create_company::MockCreateCompany::new();
            let request = create_company_request_mock();

            usecase
                .expect_create()
                .with(eq(request))
                .times(1)
                .returning(|r| Ok(create_company_from_request(&r)));

            let create_company_usecase_arc = Arc::new(usecase) as Arc<dyn CreateCompany>;

            let mut app = test::init_service(
                App::new()
                    .service(create)
                    .app_data(Data::from(create_company_usecase_arc.clone())),
            )
            .await;

            let request = test::TestRequest::post()
                .uri("/api/v1/company")
                .set_json(&create_company_request_mock())
                .to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 200);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            assert_eq!(parsed_json, get_company_json());
        }

        #[actix_web::test]
        async fn test_create_company_fails() {
            let mut usecase = create_company::MockCreateCompany::new();
            let request = create_company_request_mock();

            usecase
                .expect_create()
                .with(eq(request))
                .times(1)
                .returning(|_| {
                    Err(SqlxError::Io(std::io::Error::from(
                        std::io::ErrorKind::Other,
                    )))
                });

            let create_company_usecase_arc = Arc::new(usecase) as Arc<dyn CreateCompany>;

            let mut app = test::init_service(
                App::new()
                    .service(create)
                    .app_data(Data::from(create_company_usecase_arc.clone())),
            )
            .await;

            let request = test::TestRequest::post()
                .uri("/api/v1/company")
                .set_json(&create_company_request_mock())
                .to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 500);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            assert_eq!(
                parsed_json,
                json!("error communicating with database: other error")
            );
        }

        fn get_company_json() -> Value {
            let request = create_company_request_mock();
            let expected_company = create_company_from_request(&request);

            return serde_json::to_value(&expected_company).unwrap();
        }

        fn create_company_request_mock() -> CreateCompanyRequest {
            return CreateCompanyRequest {
                name: "company_name".to_string(),
                description: "description".to_string(),
                country_name: "country_name".to_string(),
            };
        }
    }

    mod get_companies_by_continent_service_test {
        use crate::business::use_cases::company::get_companies_by_continent;
        use crate::business::use_cases::company::get_companies_by_continent::GetCompaniesByContinent;
        use crate::http::services::company::get_by_continent;
        use crate::repositories::company_repository::CompanyDto;
        use actix_web::web::Data;
        use actix_web::{test, App};
        use mockall::predicate::*;
        use serde_json::json;
        use serde_json::Value;
        use sqlx::error::Error as SqlxError;
        use std::sync::Arc;

        #[actix_web::test]
        async fn test_get_companies_by_continent_successfully() {
            let mut usecase = get_companies_by_continent::MockGetCompaniesByContinent::new();

            let continent = "Asia".to_string();

            usecase
                .expect_get_by_continent()
                .with(eq(continent.clone()))
                .times(1)
                .returning(|c| Ok(create_companies_mock(c.to_string())));

            let get_companies_by_continent_usecase_arc =
                Arc::new(usecase) as Arc<dyn GetCompaniesByContinent>;

            let mut app = test::init_service(
                App::new()
                    .service(get_by_continent)
                    .app_data(Data::from(get_companies_by_continent_usecase_arc.clone())),
            )
            .await;

            let url = format!("/api/v1/company/continent/{}", continent.clone());

            let request = test::TestRequest::get().uri(url.as_str()).to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 200);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            assert_eq!(parsed_json, get_companies_json(continent));
        }

        #[actix_web::test]
        async fn test_get_companies_by_continent_return_no_companies() {
            let mut usecase = get_companies_by_continent::MockGetCompaniesByContinent::new();

            let continent = "Asia".to_string();

            usecase
                .expect_get_by_continent()
                .with(eq(continent.clone()))
                .times(1)
                .returning(|_| {
                    let companies: Vec<CompanyDto> = Vec::new();
                    return Ok(companies);
                });

            let get_companies_by_continent_usecase_arc =
                Arc::new(usecase) as Arc<dyn GetCompaniesByContinent>;

            let mut app = test::init_service(
                App::new()
                    .service(get_by_continent)
                    .app_data(Data::from(get_companies_by_continent_usecase_arc.clone())),
            )
            .await;

            let url = format!("/api/v1/company/continent/{}", continent.clone());

            let request = test::TestRequest::get().uri(url.as_str()).to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 200);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            let companies: Vec<CompanyDto> = Vec::new();

            assert_eq!(parsed_json, serde_json::to_value(&companies).unwrap());
        }

        #[actix_web::test]
        async fn test_get_companies_by_continent_return_error() {
            let mut usecase = get_companies_by_continent::MockGetCompaniesByContinent::new();

            let continent = "Asia".to_string();

            usecase
                .expect_get_by_continent()
                .with(eq(continent.clone()))
                .times(1)
                .returning(|_| {
                    Err(SqlxError::Io(std::io::Error::from(
                        std::io::ErrorKind::Other,
                    )))
                });

            let get_companies_by_continent_usecase_arc =
                Arc::new(usecase) as Arc<dyn GetCompaniesByContinent>;

            let mut app = test::init_service(
                App::new()
                    .service(get_by_continent)
                    .app_data(Data::from(get_companies_by_continent_usecase_arc.clone())),
            )
            .await;

            let url = format!("/api/v1/company/continent/{}", continent.clone());

            let request = test::TestRequest::get().uri(url.as_str()).to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 500);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            assert_eq!(
                parsed_json,
                json!("error communicating with database: other error")
            );
        }

        fn get_companies_json(continent: String) -> Value {
            let company = create_companies_mock(continent);
            return serde_json::to_value(&company).unwrap();
        }

        fn create_companies_mock(continent: String) -> Vec<CompanyDto> {
            return vec![CompanyDto {
                id: uuid::Uuid::nil(),
                name: "company_name".to_string(),
                description: "description".to_string(),
                country_name: "country_name".to_string(),
                created_at: None,
                updated_at: None,
                deleted_at: None,
                continent_name: continent,
            }];
        }
    }

    mod get_companies_by_country_service_test {
        use crate::business::use_cases::company::get_companies_by_country;
        use crate::business::use_cases::company::get_companies_by_country::GetCompaniesByCountry;
        use crate::http::services::company::get_by_country_name;
        use crate::repositories::company_repository::CompanyDto;
        use actix_web::web::Data;
        use actix_web::{test, App};
        use mockall::predicate::*;
        use serde_json::json;
        use serde_json::Value;
        use sqlx::error::Error as SqlxError;
        use std::sync::Arc;

        #[actix_web::test]
        async fn test_get_companies_by_country_successfully() {
            let mut usecase = get_companies_by_country::MockGetCompaniesByCountry::new();

            let country = "Japon".to_string();

            usecase
                .expect_get_by_country()
                .with(eq(country.clone()))
                .times(1)
                .returning(|c| Ok(create_companies_mock(c.to_string())));

            let get_companies_by_country_usecase_arc =
                Arc::new(usecase) as Arc<dyn GetCompaniesByCountry>;

            let mut app = test::init_service(
                App::new()
                    .service(get_by_country_name)
                    .app_data(Data::from(get_companies_by_country_usecase_arc.clone())),
            )
            .await;

            let url = format!("/api/v1/company/country/{}", country.clone());

            let request = test::TestRequest::get().uri(url.as_str()).to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 200);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            assert_eq!(parsed_json, get_companies_json(country));
        }

        #[actix_web::test]
        async fn test_get_companies_by_country_return_no_companies() {
            let mut usecase = get_companies_by_country::MockGetCompaniesByCountry::new();

            let country = "Japon".to_string();

            usecase
                .expect_get_by_country()
                .with(eq(country.clone()))
                .times(1)
                .returning(|_| {
                    let companies: Vec<CompanyDto> = Vec::new();
                    return Ok(companies);
                });

            let get_companies_by_country_usecase_arc =
                Arc::new(usecase) as Arc<dyn GetCompaniesByCountry>;

            let mut app = test::init_service(
                App::new()
                    .service(get_by_country_name)
                    .app_data(Data::from(get_companies_by_country_usecase_arc.clone())),
            )
            .await;

            let url = format!("/api/v1/company/country/{}", country.clone());

            let request = test::TestRequest::get().uri(url.as_str()).to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 200);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            let companies: Vec<CompanyDto> = Vec::new();

            assert_eq!(parsed_json, serde_json::to_value(&companies).unwrap());
        }

        #[actix_web::test]
        async fn test_get_companies_by_country_return_error() {
            let mut usecase = get_companies_by_country::MockGetCompaniesByCountry::new();

            let country = "Japon".to_string();

            usecase
                .expect_get_by_country()
                .with(eq(country.clone()))
                .times(1)
                .returning(|_| {
                    Err(SqlxError::Io(std::io::Error::from(
                        std::io::ErrorKind::Other,
                    )))
                });

            let get_companies_by_country_usecase_arc =
                Arc::new(usecase) as Arc<dyn GetCompaniesByCountry>;

            let mut app = test::init_service(
                App::new()
                    .service(get_by_country_name)
                    .app_data(Data::from(get_companies_by_country_usecase_arc.clone())),
            )
            .await;

            let url = format!("/api/v1/company/country/{}", country.clone());

            let request = test::TestRequest::get().uri(url.as_str()).to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 500);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            assert_eq!(
                parsed_json,
                json!("error communicating with database: other error")
            );
        }

        fn get_companies_json(country: String) -> Value {
            let company = create_companies_mock(country);
            return serde_json::to_value(&company).unwrap();
        }

        fn create_companies_mock(country: String) -> Vec<CompanyDto> {
            return vec![CompanyDto {
                id: uuid::Uuid::nil(),
                name: "company_name".to_string(),
                description: "description".to_string(),
                country_name: country,
                created_at: None,
                updated_at: None,
                deleted_at: None,
                continent_name: "continent".to_string(),
            }];
        }
    }

    mod get_company_by_id_service_test {
        use crate::business::entities::company::Company;
        use crate::business::use_cases::company::get_company_by_id;
        use crate::business::use_cases::company::get_company_by_id::GetCompanyByID;
        use crate::http::services::company::get_by_id;
        use actix_web::web::Data;
        use actix_web::{test, App};
        use mockall::predicate::*;
        use serde_json::json;
        use serde_json::Value;
        use sqlx::error::Error as SqlxError;
        use std::sync::Arc;

        #[actix_web::test]
        async fn test_get_company_by_id_successfully() {
            let mut usecase = get_company_by_id::MockGetCompanyByID::new();

            let id = uuid::Uuid::new_v4();

            usecase
                .expect_get_by_id()
                .with(eq(id.clone()))
                .times(1)
                .returning(|id| Ok(create_company_mock(*id)));

            let get_company_by_id_usecase_arc = Arc::new(usecase) as Arc<dyn GetCompanyByID>;

            let mut app = test::init_service(
                App::new()
                    .service(get_by_id)
                    .app_data(Data::from(get_company_by_id_usecase_arc.clone())),
            )
            .await;

            let url = format!("/api/v1/company/{}", id.to_string());

            let request = test::TestRequest::get().uri(url.as_str()).to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 200);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            assert_eq!(parsed_json, get_company_json(id));
        }

        #[actix_web::test]
        async fn test_get_company_by_id_fails_and_return_500() {
            let mut usecase = get_company_by_id::MockGetCompanyByID::new();

            let id = uuid::Uuid::new_v4();

            usecase
                .expect_get_by_id()
                .with(eq(id.clone()))
                .times(1)
                .returning(|_| {
                    Err(SqlxError::Io(std::io::Error::from(
                        std::io::ErrorKind::Other,
                    )))
                });

            let get_company_by_id_usecase_arc = Arc::new(usecase) as Arc<dyn GetCompanyByID>;

            let mut app = test::init_service(
                App::new()
                    .service(get_by_id)
                    .app_data(Data::from(get_company_by_id_usecase_arc.clone())),
            )
            .await;

            let url = format!("/api/v1/company/{}", id.to_string());

            let request = test::TestRequest::get().uri(url.as_str()).to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 500);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            assert_eq!(
                parsed_json,
                json!("error communicating with database: other error")
            );
        }

        #[actix_web::test]
        async fn test_get_company_by_id_fails_and_return_404() {
            let mut usecase = get_company_by_id::MockGetCompanyByID::new();

            let id = uuid::Uuid::new_v4();

            usecase
                .expect_get_by_id()
                .with(eq(id.clone()))
                .times(1)
                .returning(|_| Err(sqlx::Error::RowNotFound));

            let get_company_by_id_usecase_arc = Arc::new(usecase) as Arc<dyn GetCompanyByID>;

            let mut app = test::init_service(
                App::new()
                    .service(get_by_id)
                    .app_data(Data::from(get_company_by_id_usecase_arc.clone())),
            )
            .await;

            let url = format!("/api/v1/company/{}", id.to_string());

            let request = test::TestRequest::get().uri(url.as_str()).to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), 404);

            let body_bytes = test::read_body(response).await;
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let parsed_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

            assert_eq!(
                parsed_json,
                json!(format!("No company found by id: {}", id))
            );
        }

        fn get_company_json(id: uuid::Uuid) -> Value {
            let company = create_company_mock(id);

            return serde_json::to_value(&company).unwrap();
        }

        fn create_company_mock(id: uuid::Uuid) -> Company {
            return Company {
                id: id,
                name: "company_name".to_string(),
                description: "description".to_string(),
                country_name: "country_name".to_string(),
                created_at: None,
                updated_at: None,
                deleted_at: None,
            };
        }
    }
}
