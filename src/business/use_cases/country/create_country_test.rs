#[cfg(test)]
mod create_country_test {
    use crate::business::entities::country::create_country_from_request;
    use crate::business::use_cases::country::create_country as usecase;
    use crate::business::use_cases::country::create_country::CreateCountry;
    use crate::http::contracts::country::CreateCountryRequest;
    use crate::repositories::country_repository as repo;
    use mockall::predicate::*;
    use sqlx::error::Error as SqlxError;
    use std::sync::Arc;

    #[actix_web::test]
    async fn test_create_company_succesfully() {
        let mut mock_country_repository = repo::MockCountryRepository::new();
        let request = get_create_country_request_mock();
        let company_expected = create_country_from_request(&request);

        mock_country_repository
            .expect_create()
            .with(eq(create_country_from_request(&request)))
            .times(1)
            .returning(|country| Ok(country));

        let usecase = usecase::CreateCountryImpl::new(Arc::new(mock_country_repository));

        let company_created = usecase.create(&request).await;

        assert!(!company_created.is_err());
        assert_eq!(company_expected.name, company_created.unwrap().name);
    }

    #[actix_web::test]
    async fn test_get_company_by_id_from_repository_and_fails_because_random_error_on_repository() {
        let mut mock_country_repository = repo::MockCountryRepository::new();
        let request = get_create_country_request_mock();

        mock_country_repository
            .expect_create()
            .with(eq(create_country_from_request(&request)))
            .times(1)
            .returning(|_x| {
                Err(SqlxError::Io(std::io::Error::from(
                    std::io::ErrorKind::Other,
                )))
            });

        let usecase = usecase::CreateCountryImpl::new(Arc::new(mock_country_repository));

        let company_created = usecase.create(&request).await;

        assert!(company_created.is_err());
    }

    fn get_create_country_request_mock() -> CreateCountryRequest {
        return CreateCountryRequest {
            name: "Argentina".to_string(),
            continent: "South America".to_string(),
        };
    }
}
