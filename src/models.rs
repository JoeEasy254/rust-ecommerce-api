use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub category: Category,
    pub price: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub quantity: u32,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
