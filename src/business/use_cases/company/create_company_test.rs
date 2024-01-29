#[cfg(test)]
mod create_company_test {
    use crate::business::entities::company::create_company_from_request;
    use crate::business::use_cases::company::create_company as usecase;
    use crate::business::use_cases::company::create_company::CreateCompany;
    use crate::http::contracts::company::CreateCompanyRequest;
    use crate::repositories::company_repository as repo;
    use mockall::predicate::*;
    use sqlx::error::Error as SqlxError;
    use std::sync::Arc;

    #[actix_web::test]
    async fn test_create_company_succesfully() {
        let mut mock_company_repository = repo::MockCompanyRepository::new();
        let request = get_create_company_request_mock();
        let company_expected = create_company_from_request(&request);

        mock_company_repository
            .expect_create()
            .with(eq(create_company_from_request(&request)))
            .times(1)
            .returning(|company| Ok(company));

        let usecase = usecase::CreateCompanyImpl::new(Arc::new(mock_company_repository));

        let company_created = usecase.create(&request).await;

        assert!(!company_created.is_err());
        assert_eq!(company_expected.name, company_created.unwrap().name);
    }

    #[actix_web::test]
    async fn test_get_company_by_id_from_repository_and_fails_because_random_error_on_repository() {
        let mut mock_company_repository = repo::MockCompanyRepository::new();
        let request = get_create_company_request_mock();

        mock_company_repository
            .expect_create()
            .with(eq(create_company_from_request(&request)))
            .times(1)
            .returning(|_x| {
                Err(SqlxError::Io(std::io::Error::from(
                    std::io::ErrorKind::Other,
                )))
            });

            let usecase = usecase::CreateCompanyImpl::new(Arc::new(mock_company_repository));
    
            let company_created = usecase.create(&request).await;
    
            assert!(company_created.is_err());
    }

    fn get_create_company_request_mock() -> CreateCompanyRequest {
        return CreateCompanyRequest {
            name: "company_name".to_string(),
            description: "description".to_string(),
            country_name: "country_name".to_string(),
        };
    }
}
