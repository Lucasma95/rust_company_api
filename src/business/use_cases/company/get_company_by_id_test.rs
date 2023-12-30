#[cfg(test)]
mod tests {
    use crate::business::use_cases::company::get_company_by_id::GetCompanyByID;
    use crate::business::entities::company::Company;
    use crate::repositories::company_repository as repo;
    use crate::business::use_cases::company::get_company_by_id as usecase;
    use mockall::predicate::*;
    use sqlx::error::Error as SqlxError;

    #[actix_web::test]
    async fn test_get_company_from_repository_succesfully() {
        
        let mut mock_provider = repo::MockCompanyRepository::new();

        let company_id = String::from("company_id");
        let country_id = String::from("ARG");

        mock_provider.expect_get_by_id()
        .with(eq(company_id.clone()))
        .times(1)
        .returning(|company_id_mock: String | Ok(get_mock_company(company_id_mock)));
        
        let my_service = usecase::GetCompanyByIDImpl::new(Box::new(mock_provider));
        
        let company = my_service.get_by_id(company_id.clone()).await.unwrap();
        
        assert_eq!(country_id, company.country);
        assert_eq!(company_id, company.id)
    }


    #[actix_web::test]
    async fn test_get_company_from_repository_and_fails_because_random_error_on_repository() {
        
        let mut mock_provider = repo::MockCompanyRepository::new();

        let company_id = String::from("company_id");

        mock_provider.expect_get_by_id()
        .with(eq(company_id.clone()))
        .times(1)
        .returning(|_x| Err(SqlxError::Io(std::io::Error::from(std::io::ErrorKind::Other))));
        
        let my_service = usecase::GetCompanyByIDImpl::new(Box::new(mock_provider));
        
        let result = my_service.get_by_id(company_id.clone()).await;

        
        assert!(result.is_err());
    }

    fn get_mock_company(company_id: String)-> Company {

        return Company {
            id: company_id,
            description: String::from("description"),
            name: String::from("name"),
            country: String::from("ARG")
        }
    }
}
