use rusqlite::{Connection, DatabaseName, Result};
use std::{fs, path::Path};

pub fn get_connection(db_path: &Path) -> Result<Connection> {
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some(format!("Failed to create database directory: {e}")),
            )
        })?;
    }

    let conn = Connection::open(db_path)?;

    conn.pragma_update(None::<DatabaseName>, "foreign_keys", "ON")?;
    conn.pragma_update(None::<DatabaseName>, "journal_mode", "WAL")?;
    conn.pragma_update(None::<DatabaseName>, "synchronous", "NORMAL")?;

    Ok(conn)
}
