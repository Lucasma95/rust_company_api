use crate::http::contracts::company::CreateCompanyRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Decode, Encode};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize, Decode, Encode)]
pub struct Company {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub country_name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Company {
    pub fn before_create(&mut self) {
        self.id = Uuid::new_v4();
        let now = Utc::now();
        self.created_at = Some(now);
        self.updated_at = Some(now);
        self.deleted_at = None;
    }
}

pub fn create_company_from_request(request: &CreateCompanyRequest) -> Company {
    return Company {
        id: Uuid::nil(),
        name: request.name.to_owned(),
        description: request.description.to_owned(),
        country_name: request.country_name.to_owned(),
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };
}
