use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
}
