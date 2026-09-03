use rusqlite::{Connection, Result};

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        ------------------------------------------------------------
        -- ROOTS
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS tracked_roots (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            path            TEXT NOT NULL UNIQUE,
            is_enabled      INTEGER NOT NULL DEFAULT 1,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS track_ignore_roots (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            path            TEXT NOT NULL UNIQUE,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
        );

        ------------------------------------------------------------
        -- REPOSITORIES
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS repositories (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            name                TEXT NOT NULL,
            path                TEXT NOT NULL UNIQUE,
            git_dir_path        TEXT NOT NULL,
            repo_type           TEXT NOT NULL DEFAULT 'standard',
            is_enabled          INTEGER NOT NULL DEFAULT 1,
            is_starred          INTEGER NOT NULL DEFAULT 0,
            starred_at          TEXT,
            remote_url          TEXT,
            default_branch      TEXT,
            head_branch         TEXT,
            is_dirty            INTEGER NOT NULL DEFAULT 0,
            last_commit_hash    TEXT,
            last_commit_at      TEXT,
            last_scanned_at     TEXT,
            last_indexed_at     TEXT,
            index_status        TEXT,
            created_at          TEXT NOT NULL,
            updated_at          TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_repositories_path
            ON repositories(path);

        CREATE INDEX IF NOT EXISTS idx_repositories_last_commit
            ON repositories(last_commit_at);

        ------------------------------------------------------------
        -- REPOSITORY ROOTS (1-N / M-N RELATION)
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS repository_roots (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            root_id             INTEGER NOT NULL,
            repo_id             INTEGER NOT NULL,
            created_at          TEXT NOT NULL,

            FOREIGN KEY (root_id)
                REFERENCES tracked_roots(id)
                ON DELETE CASCADE,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE,

            UNIQUE(root_id, repo_id)
        );

        CREATE INDEX IF NOT EXISTS idx_repository_roots_root_id
            ON repository_roots(root_id);

        CREATE INDEX IF NOT EXISTS idx_repository_roots_repo_id
            ON repository_roots(repo_id);

        ------------------------------------------------------------
        -- FILES
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS repository_files (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            file_path           TEXT NOT NULL,
            file_name           TEXT NOT NULL,
            extension           TEXT,
            size_bytes          INTEGER,
            is_binary           INTEGER NOT NULL DEFAULT 0,
            last_modified_at    TEXT,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE,

            UNIQUE(repo_id, file_path)
        );

        CREATE INDEX IF NOT EXISTS idx_repository_files_repo
            ON repository_files(repo_id);

        CREATE INDEX IF NOT EXISTS idx_repository_files_path
            ON repository_files(repo_id, file_path);

        CREATE INDEX IF NOT EXISTS idx_repository_files_extension
            ON repository_files(extension);

        ------------------------------------------------------------
        -- BRANCHES
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS branches (
            id                          INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id                     INTEGER NOT NULL,
            name                        TEXT NOT NULL,
            is_head                     INTEGER NOT NULL DEFAULT 0,
            is_default                  INTEGER NOT NULL DEFAULT 0,
            last_commit_hash            TEXT,
            last_commit_at              TEXT,
            ahead_count_from_default    INTEGER NOT NULL DEFAULT 0,
            behind_count_from_default   INTEGER NOT NULL DEFAULT 0,
            ahead_count_from_remote     INTEGER NOT NULL DEFAULT 0,
            behind_count_from_remote    INTEGER NOT NULL DEFAULT 0,
            updated_at                  TEXT NOT NULL,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE,

            UNIQUE(repo_id, name)
        );

        CREATE INDEX IF NOT EXISTS idx_branches_repo
            ON branches(repo_id);

        ------------------------------------------------------------
        -- COMMITS
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS commits (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            hash                TEXT NOT NULL,
            author_name         TEXT,
            author_email        TEXT,
            committer_name      TEXT,
            committer_email     TEXT,
            subject             TEXT NOT NULL,
            body                TEXT,
            parent_count        INTEGER NOT NULL DEFAULT 0,
            committed_at        TEXT NOT NULL,
            inserted_at         TEXT NOT NULL,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE,

            UNIQUE(repo_id, hash)
        );

        CREATE INDEX IF NOT EXISTS idx_commits_repo
            ON commits(repo_id);

        CREATE INDEX IF NOT EXISTS idx_commits_repo_date
            ON commits(repo_id, committed_at);

        CREATE INDEX IF NOT EXISTS idx_commits_author
            ON commits(repo_id, author_email);

        ------------------------------------------------------------
        -- COMMIT GRAPH
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS commit_parents (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            commit_hash         TEXT NOT NULL,
            parent_hash         TEXT NOT NULL,
            parent_index        INTEGER NOT NULL DEFAULT 0,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE,

            UNIQUE(
                repo_id,
                commit_hash,
                parent_hash,
                parent_index
            )
        );

        CREATE INDEX IF NOT EXISTS idx_commit_parents_commit
            ON commit_parents(commit_hash);

        CREATE INDEX IF NOT EXISTS idx_commit_parents_parent
            ON commit_parents(parent_hash);

        ------------------------------------------------------------
        -- FILE CHANGES
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS commit_file_stats (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            commit_hash         TEXT NOT NULL,
            file_path           TEXT NOT NULL,
            change_type         TEXT NOT NULL,
            additions           INTEGER NOT NULL DEFAULT 0,
            deletions           INTEGER NOT NULL DEFAULT 0,
            total_changes       INTEGER NOT NULL DEFAULT 0,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_commit_file_stats_commit
            ON commit_file_stats(repo_id, commit_hash);

        CREATE INDEX IF NOT EXISTS idx_commit_file_stats_file
            ON commit_file_stats(repo_id, file_path);

        CREATE UNIQUE INDEX IF NOT EXISTS idx_commit_file_stats_unique
            ON commit_file_stats(repo_id, commit_hash, file_path);

        ------------------------------------------------------------
        -- WORKING TREE SUMMARY
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS working_tree_status (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            modified_count      INTEGER NOT NULL DEFAULT 0,
            staged_count        INTEGER NOT NULL DEFAULT 0,
            untracked_count     INTEGER NOT NULL DEFAULT 0,
            deleted_count       INTEGER NOT NULL DEFAULT 0,
            renamed_count       INTEGER NOT NULL DEFAULT 0,
            captured_at         TEXT NOT NULL,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_working_tree_repo
            ON working_tree_status(repo_id);

        ------------------------------------------------------------
        -- DAILY ACTIVITY
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS repo_activity_daily (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            activity_date       TEXT NOT NULL,
            commit_count        INTEGER NOT NULL DEFAULT 0,
            additions           INTEGER NOT NULL DEFAULT 0,
            deletions           INTEGER NOT NULL DEFAULT 0,
            files_changed       INTEGER NOT NULL DEFAULT 0,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE,

            UNIQUE(repo_id, activity_date)
        );

        ------------------------------------------------------------
        -- HOTSPOTS
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS file_hotspots (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            file_path           TEXT NOT NULL,
            touch_count         INTEGER NOT NULL DEFAULT 0,
            churn_score         REAL NOT NULL DEFAULT 0,
            hotspot_score       REAL NOT NULL DEFAULT 0,
            last_touched_at     TEXT,
            updated_at          TEXT NOT NULL,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE,

            UNIQUE(repo_id, file_path)
        );

        CREATE INDEX IF NOT EXISTS idx_hotspots_score
            ON file_hotspots(repo_id, hotspot_score DESC);

        ------------------------------------------------------------
        -- CONTRIBUTORS
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS contributors (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            author_name         TEXT NOT NULL,
            author_email        TEXT,
            commit_count        INTEGER NOT NULL DEFAULT 0,
            additions           INTEGER NOT NULL DEFAULT 0,
            deletions           INTEGER NOT NULL DEFAULT 0,
            active_days         INTEGER NOT NULL DEFAULT 0,
            last_commit_at      TEXT,
            updated_at          TEXT NOT NULL,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE,

            UNIQUE(repo_id, author_email)
        );

        ------------------------------------------------------------
        -- HEALTH
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS repository_health (
            repo_id             INTEGER PRIMARY KEY,
            health_score        REAL NOT NULL DEFAULT 0,
            issues_count        INTEGER NOT NULL DEFAULT 0,
            warnings_count      INTEGER NOT NULL DEFAULT 0,
            check_status        TEXT NOT NULL DEFAULT 'pending',
            last_check_at       TEXT NOT NULL,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE
        );

        ------------------------------------------------------------
        -- INDEXING JOBS
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS indexing_jobs (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            job_type            TEXT NOT NULL,
            status              TEXT NOT NULL DEFAULT 'pending',
            progress            INTEGER NOT NULL DEFAULT 0,
            total_items         INTEGER,
            processed_items     INTEGER NOT NULL DEFAULT 0,
            error_message       TEXT,
            started_at          TEXT,
            completed_at        TEXT,
            created_at          TEXT NOT NULL,
            updated_at          TEXT NOT NULL,

            FOREIGN KEY (repo_id)
                REFERENCES repositories(id)
                ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_indexing_jobs_repo
            ON indexing_jobs(repo_id);

        CREATE INDEX IF NOT EXISTS idx_indexing_jobs_status
            ON indexing_jobs(status);

        ------------------------------------------------------------
        -- AUDIT LOGS
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS audit_logs (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            action              TEXT NOT NULL,
            entity_type         TEXT NOT NULL,
            entity_id           TEXT NOT NULL,
            details             TEXT,
            created_at          TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_audit_logs_action
            ON audit_logs(action, created_at);

        ------------------------------------------------------------
        -- SETTINGS
        ------------------------------------------------------------

        CREATE TABLE IF NOT EXISTS settings (
            key                 TEXT PRIMARY KEY,
            value               TEXT NOT NULL,
            updated_at          TEXT NOT NULL
        );

        ------------------------------------------------------------
        -- SEARCH (FTS5) — cross-entity searchable text index
        ------------------------------------------------------------
        -- Stores searchable text produced during repository sync for multiple
        -- entity kinds (commit / contributor / file / branch), keyed by repo.
        -- NOTE: FTS5 virtual tables don't support ON CONFLICT, so sync rebuilds
        -- each repo's entries with a DELETE-then-INSERT pattern. The rowid is a
        -- monotonically increasing counter (auto-assigned on INSERT).
        -- DROP is intentional: search_index was unused dead schema; recreating it
        -- with the correct columns ensures a consistent definition on every start.
        DROP TABLE IF EXISTS search_index;
        CREATE VIRTUAL TABLE IF NOT EXISTS search_index
        USING fts5(
            repo_id UNINDEXED,
            entity_type UNINDEXED,
            entity_id UNINDEXED,
            title,
            body,
            tokenize = 'unicode61 remove_diacritics 2'
        );
        "#,
    )?;

    // Handle migration for existing databases if upgrading
    let mut stmt = conn.prepare("PRAGMA table_info(repositories)")?;
    let columns = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    if columns.iter().any(|c| c == "root_id") {
        // 1. Copy legacy root_id links into repository_roots
        let _ = conn.execute(
            "INSERT OR IGNORE INTO repository_roots (root_id, repo_id, created_at)
             SELECT root_id, id, created_at FROM repositories WHERE root_id IS NOT NULL",
            [],
        );

        // 2. Recreate repositories table without root_id to ensure NOT NULL constraint on root_id is completely removed across all SQLite versions
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = OFF;

            CREATE TABLE repositories_dg_tmp (
                id                  INTEGER PRIMARY KEY AUTOINCREMENT,
                name                TEXT NOT NULL,
                path                TEXT NOT NULL UNIQUE,
                git_dir_path        TEXT NOT NULL,
                repo_type           TEXT NOT NULL DEFAULT 'standard',
                is_enabled          INTEGER NOT NULL DEFAULT 1,
                is_starred          INTEGER NOT NULL DEFAULT 0,
                starred_at          TEXT,
                remote_url          TEXT,
                default_branch      TEXT,
                head_branch         TEXT,
                is_dirty            INTEGER NOT NULL DEFAULT 0,
                last_commit_hash    TEXT,
                last_commit_at      TEXT,
                last_scanned_at     TEXT,
                last_indexed_at     TEXT,
                index_status        TEXT,
                created_at          TEXT NOT NULL,
                updated_at          TEXT NOT NULL
            );

            INSERT INTO repositories_dg_tmp (
                id, name, path, git_dir_path, repo_type,
                is_enabled, is_starred, starred_at, remote_url, default_branch, head_branch,
                is_dirty, last_commit_hash, last_commit_at, last_scanned_at,
                last_indexed_at, index_status, created_at, updated_at
            )
            SELECT 
                id, name, path, git_dir_path, repo_type,
                1, 0, NULL, remote_url, default_branch, head_branch,
                is_dirty, last_commit_hash, last_commit_at, last_scanned_at,
                last_indexed_at, index_status, created_at, updated_at
            FROM repositories;

            DROP TABLE repositories;
            ALTER TABLE repositories_dg_tmp RENAME TO repositories;

            CREATE INDEX IF NOT EXISTS idx_repositories_path ON repositories(path);
            CREATE INDEX IF NOT EXISTS idx_repositories_last_commit ON repositories(last_commit_at);

            PRAGMA foreign_keys = ON;
            "#,
        )?;
    } else if !columns.is_empty() && !columns.iter().any(|c| c == "is_enabled") {
        let _ = conn.execute(
            "ALTER TABLE repositories ADD COLUMN is_enabled INTEGER NOT NULL DEFAULT 1",
            [],
        );
    }

    // Add starred columns if they don't exist
    let mut stmt = conn.prepare("PRAGMA table_info(repositories)")?;
    let current_columns = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    let added_starred = if !current_columns.iter().any(|c| c == "is_starred") {
        let result = conn.execute(
            "ALTER TABLE repositories ADD COLUMN is_starred INTEGER NOT NULL DEFAULT 0",
            [],
        );
        result.is_ok()
    } else {
        true
    };

    let added_starred_at = if !current_columns.iter().any(|c| c == "starred_at") {
        let result = conn.execute(
            "ALTER TABLE repositories ADD COLUMN starred_at TEXT",
            [],
        );
        result.is_ok()
    } else {
        true
    };

    // Add index for starred repositories if it doesn't exist
    // Only create index if columns exist
    if (current_columns.iter().any(|c| c == "is_starred") || added_starred) 
        && (current_columns.iter().any(|c| c == "starred_at") || added_starred_at) {
        let _ = conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_repositories_starred ON repositories(is_starred, starred_at DESC)",
            [],
        );
    }

    // Ensure indexes and views referencing new columns are created
    conn.execute_batch(
        r#"
        CREATE INDEX IF NOT EXISTS idx_repositories_is_enabled
            ON repositories(is_enabled);

        DROP VIEW IF EXISTS repository_summary;

        CREATE VIEW IF NOT EXISTS repository_summary AS
        SELECT
            r.id,
            r.name,
            r.path,
            r.git_dir_path,
            r.repo_type,
            r.is_enabled,
            r.is_starred,
            r.starred_at,
            h.health_score,
            r.default_branch,
            r.head_branch,
            r.remote_url,
            r.is_dirty,
            r.last_commit_hash,
            r.last_commit_at,
            r.last_scanned_at,
            r.last_indexed_at,
            r.index_status,
            r.created_at,
            r.updated_at,
            c.total_commits,
            c.weekly_commits,
            c.unique_contributors
        FROM repositories r
        LEFT JOIN repository_health h
            ON h.repo_id = r.id
        LEFT JOIN (
            SELECT 
                repo_id, 
                COUNT(*) as total_commits, 
                COUNT(DISTINCT author_email) as unique_contributors,
                SUM(CASE WHEN committed_at >= date('now', '-7 days') THEN 1 ELSE 0 END) as weekly_commits
            FROM commits
            GROUP BY repo_id
        ) c ON c.repo_id = r.id;
        "#,
    )?;

    // Dedicated FTS5 search index for repositories.
    // Only identity data needed for searching is stored (name/path/remote_url).
    // rowid == repositories.id so deletes/updates are trivial via triggers.
    conn.execute_batch(
        r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS repo_search USING fts5(
            name,
            path,
            remote_url,
            repo_id UNINDEXED,
            tokenize = 'unicode61 remove_diacritics 2'
        );

        CREATE TRIGGER IF NOT EXISTS repo_search_ai AFTER INSERT ON repositories BEGIN
            INSERT INTO repo_search(rowid, repo_id, name, path, remote_url)
            VALUES (new.id, new.id, new.name, new.path, COALESCE(new.remote_url, ''));
        END;

        CREATE TRIGGER IF NOT EXISTS repo_search_ad AFTER DELETE ON repositories BEGIN
            DELETE FROM repo_search WHERE rowid = old.id;
        END;

        CREATE TRIGGER IF NOT EXISTS repo_search_au AFTER UPDATE ON repositories BEGIN
            DELETE FROM repo_search WHERE rowid = old.id;
            INSERT INTO repo_search(rowid, repo_id, name, path, remote_url)
            VALUES (new.id, new.id, new.name, new.path, COALESCE(new.remote_url, ''));
        END;

        -- Backfill any repositories that predate this migration (guards against
        -- the legacy table-recreate path and duplicate inserts). FTS5 virtual
        -- tables don't support ON CONFLICT, so use a NOT EXISTS guard instead.
        INSERT INTO repo_search(rowid, repo_id, name, path, remote_url)
        SELECT id, id, name, path, COALESCE(remote_url, '') FROM repositories
        WHERE id NOT IN (SELECT rowid FROM repo_search);
        "#,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_from_old_schema() {
        let conn = Connection::open_in_memory().unwrap();

        // Setup legacy schema (repositories with root_id and without is_enabled)
        conn.execute_batch(
            r#"
            CREATE TABLE tracked_roots (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                is_enabled INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE repositories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                root_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                git_dir_path TEXT NOT NULL,
                repo_type TEXT NOT NULL DEFAULT 'standard',
                remote_url TEXT,
                default_branch TEXT,
                head_branch TEXT,
                is_dirty INTEGER NOT NULL DEFAULT 0,
                last_commit_hash TEXT,
                last_commit_at TEXT,
                last_scanned_at TEXT,
                last_indexed_at TEXT,
                index_status TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            INSERT INTO tracked_roots (id, path, is_enabled, created_at, updated_at)
            VALUES (1, '/home/user/projects', 1, '2026-09-01T10:00:00Z', '2026-09-01T10:00:00Z');

            INSERT INTO repositories (id, root_id, name, path, git_dir_path, repo_type, is_dirty, created_at, updated_at)
            VALUES (1, 1, 'old-repo', '/home/user/projects/old-repo', '/home/user/projects/old-repo/.git', 'standard', 0, '2026-09-01T10:00:00Z', '2026-09-01T10:00:00Z');
            "#,
        ).unwrap();

        // Run migrations
        run_migrations(&conn).unwrap();

        // Verify repository_roots has the migrated link
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM repository_roots WHERE root_id = 1 AND repo_id = 1", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 1);

        // Verify is_enabled was added with default 1
        let is_enabled: i64 = conn.query_row("SELECT is_enabled FROM repositories WHERE id = 1", [], |r| r.get(0)).unwrap();
        assert_eq!(is_enabled, 1);

        // Verify view works properly
        let view_count: i64 = conn.query_row("SELECT COUNT(*) FROM repository_summary WHERE is_enabled = 1", [], |r| r.get(0)).unwrap();
        assert_eq!(view_count, 1);
    }
}

