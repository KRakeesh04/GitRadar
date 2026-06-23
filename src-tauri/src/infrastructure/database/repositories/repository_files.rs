use crate::infrastructure::database::models::RepositoryFile;
use rusqlite::{params, Connection, Result};

pub fn insert_repository_file(
    conn: &Connection,
    repo_id: i64,
    file_path: &str,
    file_name: &str,
    extension: Option<&str>,
    size_bytes: Option<i64>,
    is_binary: bool,
    last_modified_at: Option<&str>,
) -> Result<i64> {
    conn.execute(
        r#"
        INSERT INTO repository_files (
            repo_id, file_path, file_name, extension, size_bytes, is_binary, last_modified_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        params![
            repo_id,
            file_path,
            file_name,
            extension,
            size_bytes,
            is_binary,
            last_modified_at
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn upsert_repository_file(
    conn: &Connection,
    repo_id: i64,
    file_path: &str,
    file_name: &str,
    extension: Option<&str>,
    size_bytes: Option<i64>,
    is_binary: bool,
    last_modified_at: Option<&str>,
) -> Result<i64> {
    conn.execute(
        r#"
        INSERT OR REPLACE INTO repository_files (
            repo_id, file_path, file_name, extension, size_bytes, is_binary, last_modified_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        params![
            repo_id,
            file_path,
            file_name,
            extension,
            size_bytes,
            is_binary,
            last_modified_at
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_repository_files(conn: &Connection, repo_id: i64) -> Result<Vec<RepositoryFile>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, file_path, file_name, extension, size_bytes, is_binary, last_modified_at 
        FROM repository_files 
        WHERE repo_id = ?1 
        ORDER BY file_path
        "#,
    )?;
    let files = stmt.query_map([repo_id], |row| {
        Ok(RepositoryFile {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            file_path: row.get(2)?,
            file_name: row.get(3)?,
            extension: row.get(4)?,
            size_bytes: row.get(5)?,
            is_binary: row.get(6)?,
            last_modified_at: row.get(7)?,
        })
    })?;
    Ok(files.filter_map(Result::ok).collect())
}

pub fn get_repository_file_by_path(
    conn: &Connection,
    repo_id: i64,
    file_path: &str,
) -> Result<Option<RepositoryFile>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, file_path, file_name, extension, size_bytes, is_binary, last_modified_at 
        FROM repository_files 
        WHERE repo_id = ?1 AND file_path = ?2
        "#,
    )?;
    let file = stmt.query_row(params![repo_id, file_path], |row| {
        Ok(RepositoryFile {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            file_path: row.get(2)?,
            file_name: row.get(3)?,
            extension: row.get(4)?,
            size_bytes: row.get(5)?,
            is_binary: row.get(6)?,
            last_modified_at: row.get(7)?,
        })
    });
    match file {
        Ok(f) => Ok(Some(f)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn delete_repository_files(conn: &Connection, repo_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM repository_files WHERE repo_id = ?1",
        params![repo_id],
    )?;
    Ok(())
}

pub fn get_files_by_extension(
    conn: &Connection,
    repo_id: i64,
    extension: &str,
) -> Result<Vec<RepositoryFile>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, file_path, file_name, extension, size_bytes, is_binary, last_modified_at 
        FROM repository_files 
        WHERE repo_id = ?1 AND extension = ?2 
        ORDER BY file_path
        "#,
    )?;
    let files = stmt.query_map(params![repo_id, extension], |row| {
        Ok(RepositoryFile {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            file_path: row.get(2)?,
            file_name: row.get(3)?,
            extension: row.get(4)?,
            size_bytes: row.get(5)?,
            is_binary: row.get(6)?,
            last_modified_at: row.get(7)?,
        })
    })?;
    Ok(files.filter_map(Result::ok).collect())
}
