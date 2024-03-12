use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub uuid: Option<Uuid>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub user_uuid: Option<Uuid>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
