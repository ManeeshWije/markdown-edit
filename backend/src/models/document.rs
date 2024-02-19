use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub uuid: String,
    pub title: String,
    pub content: String,
    pub user_uuid: String,
}
