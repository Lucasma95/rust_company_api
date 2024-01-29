#[cfg(test)]
mod get_companies_by_id_test {
    use std::sync::Arc;

    use crate::business::entities::company::Company;
    use crate::business::use_cases::company::get_company_by_id as usecase;
    use crate::business::use_cases::company::get_company_by_id::GetCompanyByID;
    use crate::repositories::company_repository as repo;
    use mockall::predicate::*;
    use sqlx::error::Error as SqlxError;
    use sqlx::types::chrono::Utc;

    #[actix_web::test]
    async fn test_get_company_by_id_from_repository_succesfully() {
        let mut mock_company_repository = repo::MockCompanyRepository::new();

        let company_id = uuid::Uuid::new_v4();
        let company_expected = get_mock_company(&company_id);

        mock_company_repository
            .expect_get_by_id()
            .with(eq(company_id))
            .times(1)
            .returning(|company_id_mock| Ok(get_mock_company(&company_id_mock)));

        let usecase = usecase::GetCompanyByIDImpl::new(Arc::new(mock_company_repository));

        let company = usecase.get_by_id(&company_id).await.unwrap();

        assert_eq!(company_expected.id, company.id);
        assert_eq!(company_expected.country_name, company.country_name)
    }

    #[actix_web::test]
    async fn test_get_company_by_id_from_repository_and_fails_because_random_error_on_repository() {
        let mut mock_provider = repo::MockCompanyRepository::new();

        let company_id = uuid::Uuid::new_v4();

        mock_provider
            .expect_get_by_id()
            .with(eq(company_id))
            .times(1)
            .returning(|_x| {
                Err(SqlxError::Io(std::io::Error::from(
                    std::io::ErrorKind::Other,
                )))
            });

        let usecase = usecase::GetCompanyByIDImpl::new(Arc::new(mock_provider));

        let result = usecase.get_by_id(&company_id).await;

        assert!(result.is_err());
    }

    fn get_mock_company(company_id: &uuid::Uuid) -> Company {
        return Company {
            id: *company_id,
            description: String::from("description"),
            name: String::from("name"),
            country_name: String::from("Argentina"),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            deleted_at: None,
        };
    }
}
