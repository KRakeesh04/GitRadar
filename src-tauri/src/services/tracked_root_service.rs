use std::path::PathBuf;

use crate::{
    domain::{tracked_root::RootId, DomainError, DomainResult, TrackedRoot},
    infrastructure::database::repositories::tracked_roots,
};
use rusqlite::Connection;

pub fn add_tracked_root_path(conn: &mut Connection, path: &str) -> DomainResult<i64> {
    let added = match tracked_roots::insert_tracked_root(conn, path, true) {
        Ok(added_id) => added_id,
        Err(e) => return Err(DomainError::AddOrDeleteTrackedRootPathFailed(e.to_string())),
    };

    Ok(added)
}

pub fn get_all_tracked_root_paths(conn: &Connection) -> DomainResult<Vec<TrackedRoot>> {
    let all_paths = match tracked_roots::get_all_tracked_roots(conn) {
        Ok(paths) => paths,
        Err(e) => return Err(DomainError::GetTrackedRootPathsFailed(e.to_string())),
    };

    let mut paths = Vec::new();
    for path in all_paths {
        paths.push(TrackedRoot::new(
            RootId(path.id),
            PathBuf::from(path.path),
            path.is_enabled,
            path.updated_at,
        )?);
    }

    Ok(paths)
}

pub fn enable_or_disable_track_root_path(
    conn: &mut Connection,
    path: &str,
    enabled: bool,
) -> DomainResult<bool> {
    match tracked_roots::update_tracked_root_enabled(conn, path, enabled) {
        Ok(updated) => Ok(updated),
        Err(e) => Err(DomainError::EnableTrackedRootPathFailed(e.to_string())),
    }
}

pub fn delete_tracked_root_path(conn: &mut Connection, root_id: i64) -> DomainResult<bool> {
    match tracked_roots::delete_tracked_root(conn, root_id) {
        Ok(deleted) => Ok(deleted),
        Err(e) => Err(DomainError::AddOrDeleteTrackedRootPathFailed(e.to_string())),
    }
}
