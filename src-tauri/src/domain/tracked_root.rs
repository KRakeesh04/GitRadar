use std::path::PathBuf;

use crate::domain::DomainResult;

#[derive(Debug, Clone)]
pub struct TrackedRoot {
    pub id: RootId,
    pub path: PathBuf,
    pub is_enabled: bool,
    pub updated_at: String,
}

impl TrackedRoot {
    pub fn new(
        id: RootId,
        path: PathBuf,
        is_enabled: bool,
        updated_at: String,
    ) -> DomainResult<Self> {
        Ok(TrackedRoot {
            id,
            path,
            is_enabled,
            updated_at,
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RootId(pub i64);
