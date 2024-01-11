use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Company {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub country: String,
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
