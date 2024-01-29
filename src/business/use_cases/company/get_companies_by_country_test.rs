#[cfg(test)]
mod get_companies_by_country_test {
    use crate::business::use_cases::company::get_companies_by_country as usecase;
    use crate::business::use_cases::company::get_companies_by_country::GetCompaniesByCountry;
    use crate::repositories::company_repository as repo;
    use mockall::predicate::*;
    use sqlx::error::Error as SqlxError;
    use sqlx::types::chrono::Utc;
    use std::sync::Arc;

    #[actix_web::test]
    async fn test_get_company_by_country_from_repository_succesfully() {
        let mut mock_company_repository = repo::MockCompanyRepository::new();

        let country_name = "Argentina";
        let companies_expected = get_mock_companies_dto(&country_name);

        mock_company_repository
            .expect_get_by_country()
            .with(eq(country_name))
            .times(1)
            .returning(|country_name| Ok(get_mock_companies_dto(country_name)));

        let usecase = usecase::GetCompaniesByCountryImpl::new(Arc::new(mock_company_repository));

        let companies = usecase.get_by_country(&country_name).await.unwrap();
        
        assert_eq!(
            companies_expected[0].country_name,
            companies[0].country_name
        )
    }

    #[actix_web::test]
    async fn test_get_company_by_country_from_repository_and_fails_because_random_error_on_repository(
    ) {
        let mut mock_company_repository = repo::MockCompanyRepository::new();

        let country_name = "Argentina";

        mock_company_repository
            .expect_get_by_country()
            .with(eq(country_name))
            .times(1)
            .returning(|_x| {
                Err(SqlxError::Io(std::io::Error::from(
                    std::io::ErrorKind::Other,
                )))
            });

        let usecase = usecase::GetCompaniesByCountryImpl::new(Arc::new(mock_company_repository));

        let companies = usecase.get_by_country(&country_name).await;

        assert!(companies.is_err());
    }

    fn get_mock_companies_dto(country_name: &str) -> Vec<repo::CompanyDto> {
        return vec![repo::CompanyDto {
            id: uuid::Uuid::new_v4(),
            description: String::from("description"),
            name: String::from("name"),
            country_name: country_name.to_owned(),
            continent_name: String::from("South America"),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }];
    }
}
