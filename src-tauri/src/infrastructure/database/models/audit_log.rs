use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i64,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub details: Option<String>,
    pub created_at: String,
}
