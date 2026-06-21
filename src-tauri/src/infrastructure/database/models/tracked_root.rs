use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackedRoot {
    pub id: i64,
    pub path: String,
    pub is_enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackIgnoreRoot {
    pub id: i64,
    pub path: String,
    pub created_at: String,
    pub updated_at: String,
}
