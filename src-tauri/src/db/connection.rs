use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

pub fn get_connection() -> Result<Connection> {
    let db_path = get_db_path();
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some(format!("Failed to create database directory: {}", e)),
            )
        })?;
    }

    let conn = Connection::open(db_path)?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    conn.execute("PRAGMA journal_mode = WAL;", [])?;
    conn.execute("PRAGMA synchronous = NORMAL;", [])?;
    Ok(conn)
}

fn get_db_path() -> PathBuf {
    // Use proper app data directory for desktop applications
    let app_data = if cfg!(target_os = "macos") {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("GitRadar")
    } else if cfg!(target_os = "windows") {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("GitRadar")
    } else {
        // Linux and others
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("gitradar")
    };
    
    app_data.join("gitradar.db")
}