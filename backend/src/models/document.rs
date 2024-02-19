use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub uuid: Uuid,
    pub title: String,
    pub content: String,
    pub user_uuid: Uuid,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
