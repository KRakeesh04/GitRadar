use rusqlite::Connection;

use crate::{
    domain::{DomainError, DomainResult},
    infrastructure::database::{
        models::search_index::SearchHit,
        repositories::{
            branches, commits, contributors, repository_files, repositories, search_index,
        },
    },
};

/// Search the cross-entity searchable-text index (commits, contributors, files,
/// branches) and return paginated hits resolved against repository names.
pub fn search(
    conn: &Connection,
    query: &str,
    limit: usize,
    offset: usize,
) -> DomainResult<Vec<SearchHit>> {
    search_index::search(conn, query, limit, offset)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to search index: {e}")))
}

/// Rebuild the searchable-text index for every repository from the existing
/// database records. Runs at app startup so the search index is always
/// populated even if no full sync has happened yet.
pub fn rebuild_search_index_from_db(conn: &mut Connection) -> DomainResult<usize> {
    let all_repos = repositories::get_all_repositories(conn, 5000, 0)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to load repositories: {e}")))?
        .unwrap_or_default();

    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;

    let mut total_entries = 0usize;

    for repo in &all_repos {
        search_index::clear_repo_entries(&tx, repo.id).map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to clear search index: {e}"))
        })?;

        // Commits: title = subject, body = author + email + subject + body
        if let Ok(Some(repo_commits)) = commits::get_commits_by_repo(&tx, repo.id, 5000, 0) {
            for c in &repo_commits {
                let mut body = String::new();
                if let Some(ref name) = c.author_name {
                    body.push_str(name);
                    body.push(' ');
                }
                if let Some(ref email) = c.author_email {
                    body.push_str(email);
                    body.push(' ');
                }
                body.push_str(&c.subject);
                if let Some(ref b) = c.body {
                    body.push(' ');
                    body.push_str(b);
                }
                search_index::insert_entry(&tx, repo.id, "commit", c.id, &c.subject, &body)
                    .map_err(|e| {
                        DomainError::InvalidRepository(format!("Failed to index commit: {e}"))
                    })?;
                total_entries += 1;
            }
        }

        // Contributors: title = name, body = email
        if let Ok(contribs) = contributors::get_contributors_by_repo(&tx, repo.id) {
            for c in &contribs {
                let body = c.author_email.as_deref().unwrap_or("");
                search_index::insert_entry(&tx, repo.id, "contributor", c.id, &c.author_name, body)
                    .map_err(|e| {
                        DomainError::InvalidRepository(format!("Failed to index contributor: {e}"))
                    })?;
                total_entries += 1;
            }
        }

        // Files: title = file_name, body = file_path + extension
        if let Ok(files) = repository_files::get_repository_files(&tx, repo.id) {
            for f in &files {
                let mut body = f.file_path.clone();
                if let Some(ref ext) = f.extension {
                    body.push(' ');
                    body.push_str(ext);
                }
                search_index::insert_entry(&tx, repo.id, "file", f.id, &f.file_name, &body)
                    .map_err(|e| {
                        DomainError::InvalidRepository(format!("Failed to index file: {e}"))
                    })?;
                total_entries += 1;
            }
        }

        // Branches: title = name, body = name
        if let Ok(Some(branch_rows)) = branches::get_all_branches(&tx, repo.id) {
            for b in &branch_rows {
                search_index::insert_entry(&tx, repo.id, "branch", b.id, &b.name, &b.name)
                    .map_err(|e| {
                        DomainError::InvalidRepository(format!("Failed to index branch: {e}"))
                    })?;
                total_entries += 1;
            }
        }
    }

    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit search index rebuild: {e}"))
    })?;

    Ok(total_entries)
}
