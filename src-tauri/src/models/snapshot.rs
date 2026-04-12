use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: i64,
    pub repo_id: i64,
    pub snapshot_type: String,
    pub snapshot_key: String,
    pub data_json: String,
    pub created_at: String,
}
