use serde::{Deserialize, Serialize};

/// A lightweight projection of a row in the FTS5 `search_index` virtual table.
///
/// `SearchIndexEntry` represents a single searchable "document" produced during
/// repository sync. `entity_type` distinguishes the kind of source record
/// (e.g. "commit", "contributor", "file", "branch") and `entity_id` points back
/// to the source record so a match can be resolved to real data.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIndexEntry {
    pub repo_id: i64,
    pub entity_type: String,
    pub entity_id: i64,
    pub title: String,
    pub body: String,
}

/// A search hit resolved with enough context to render in the UI and link back
/// to the repository it belongs to.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchHit {
    pub repo_id: i64,
    pub repo_name: String,
    pub entity_type: String,
    pub entity_id: i64,
    pub title: String,
    pub body: String,
}
