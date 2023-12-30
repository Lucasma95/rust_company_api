use crate::http::contracts::company::CreateCompanyRequest;
use std::error::Error;

pub trait SaveCompany: Send + Sync {
    fn save(&self, request: &CreateCompanyRequest) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone, Copy, Default)]
pub struct SaveCompanyImpl;

impl SaveCompanyImpl {
    pub fn new() -> SaveCompanyImpl {
        SaveCompanyImpl{}
    }
}

impl SaveCompany for SaveCompanyImpl {
    fn save(&self, request: &CreateCompanyRequest) -> Result<(), Box<dyn Error>> {
        println!("name: {}", request.name);
        println!("description: {}", request.description);
        println!("country: {}", request.country);
        Ok(())
    }
}
