use crate::infrastructure::database::models::search_index::SearchHit;
use rusqlite::{params, Connection, Result};

/// Remove every searchable-text entry for a repository. Used at the start of a
/// sync so the index reflects the current state (FTS5 virtual tables don't
/// support ON CONFLICT, so we rebuild per-repo with delete-then-insert).
pub fn clear_repo_entries(conn: &Connection, repo_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM search_index WHERE repo_id = ?1",
        params![repo_id],
    )?;
    Ok(())
}

/// Insert a single searchable-text document. The FTS5 rowid is auto-assigned.
pub fn insert_entry(
    conn: &Connection,
    repo_id: i64,
    entity_type: &str,
    entity_id: i64,
    title: &str,
    body: &str,
) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO search_index (repo_id, entity_type, entity_id, title, body)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        params![repo_id, entity_type, entity_id, title, body],
    )?;
    Ok(())
}

/// Build an FTS5 MATCH expression from a user query. Each whitespace-separated
/// token becomes a prefix match (`tok*`) combined with implicit AND, which gives
/// fast, forgiving "starts-with" behaviour across title/body for every entity.
fn build_match_expression(query: &str) -> String {
    query
        .split_whitespace()
        .map(|token| format!("{}*", token.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Search the cross-entity text index, resolved against repository name.
/// Returns up to `limit` hits starting at `offset`.
pub fn search(
    conn: &Connection,
    query: &str,
    limit: usize,
    offset: usize,
) -> Result<Vec<SearchHit>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let match_expr = build_match_expression(trimmed);

    let mut stmt = conn.prepare(
        r#"
        SELECT
            si.repo_id,
            COALESCE(r.name, ''),
            si.entity_type,
            si.entity_id,
            si.title,
            si.body
        FROM search_index si
        LEFT JOIN repositories r ON r.id = si.repo_id
        WHERE search_index MATCH ?1
        ORDER BY rank
        LIMIT ?2 OFFSET ?3
        "#,
    )?;

    // NB: the FTS5 "search_index MATCH ?1" predicate must reference the table
    // name; the column aliases above come from the joined repositories row.
    let hits = stmt
        .query_map(
            params![match_expr, limit as i64, offset as i64],
            |row| {
                Ok(SearchHit {
                    repo_id: row.get(0)?,
                    repo_name: row.get(1)?,
                    entity_type: row.get(2)?,
                    entity_id: row.get(3)?,
                    title: row.get(4)?,
                    body: row.get(5)?,
                })
            },
        )?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(hits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database::migrations::run_migrations;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_search_index_insert_and_search() {
        let conn = setup_test_db();

        // Insert some cross-entity searchable documents directly.
        insert_entry(&conn, 1, "commit", 0, "Fix login crash", "alice alice@x.com Fix login crash").unwrap();
        insert_entry(&conn, 1, "contributor", 0, "Bob", "bob@x.com").unwrap();
        insert_entry(&conn, 2, "file", 5, "main.rs", "src/main.rs rs").unwrap();
        insert_entry(&conn, 2, "branch", 7, "feature-search", "feature-search").unwrap();

        // Match across title/body via prefix tokens.
        let hits = search(&conn, "login", 50, 0).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].entity_type, "commit");
        assert_eq!(hits[0].title, "Fix login crash");

        let hits = search(&conn, "bob", 50, 0).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].entity_type, "contributor");
        assert_eq!(hits[0].repo_name, ""); // no repository row exists, still matched

        let hits = search(&conn, "main", 50, 0).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].entity_type, "file");

        // Empty query returns nothing.
        let hits = search(&conn, "   ", 50, 0).unwrap();
        assert_eq!(hits.len(), 0);

        // Clearing removes all entries for a repo.
        clear_repo_entries(&conn, 2).unwrap();
        let hits = search(&conn, "main", 50, 0).unwrap();
        assert_eq!(hits.len(), 0);
    }
}
