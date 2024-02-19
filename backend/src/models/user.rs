use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
