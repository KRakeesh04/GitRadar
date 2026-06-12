# GitRadar Database Schema

Version: 2.0

---

# Database Principles

Goals:

* Fast startup
* Fast dashboard loading
* Incremental indexing
* Commit graph support
* Secure local storage
* Analytics caching
* Future extensibility

---

# Core Tables

## tracked_roots

Stores user-approved root directories.

```sql
tracked_roots
```

| Column     | Type        |
| ---------- | ----------- |
| id         | INTEGER PK  |
| name       | TEXT        |
| path       | TEXT UNIQUE |
| is_enabled | BOOLEAN     |
| created_at | DATETIME    |
| updated_at | DATETIME    |

---

## tracked_repositories

Stores discovered repositories.

| Column                   | Type        |
| ------------------------ | ----------- |
| id                       | INTEGER PK  |
| root_id                  | INTEGER FK  |
| name                     | TEXT        |
| path                     | TEXT UNIQUE |
| git_dir_path             | TEXT        |
| default_branch           | TEXT        |
| head_branch              | TEXT        |
| repository_size_bytes    | INTEGER     |
| last_activity_at         | DATETIME    |
| last_scanned_at          | DATETIME    |
| last_indexed_commit_hash | TEXT        |
| is_enabled               | BOOLEAN     |
| created_at               | DATETIME    |
| updated_at               | DATETIME    |

Indexes:

```sql
idx_repo_root
idx_repo_path
idx_repo_activity
```

---

# Branch Tables

## branches

| Column           | Type       |
| ---------------- | ---------- |
| id               | INTEGER PK |
| repo_id          | INTEGER FK |
| name             | TEXT       |
| is_head          | BOOLEAN    |
| is_remote        | BOOLEAN    |
| upstream_branch  | TEXT       |
| last_commit_hash | TEXT       |
| last_commit_at   | DATETIME   |

---

# Commit Tables

## commits

| Column          | Type       |
| --------------- | ---------- |
| id              | INTEGER PK |
| repo_id         | INTEGER FK |
| hash            | TEXT       |
| author_name     | TEXT       |
| author_email    | TEXT       |
| committer_name  | TEXT       |
| committer_email | TEXT       |
| message_subject | TEXT       |
| message_body    | TEXT       |
| committed_at    | DATETIME   |
| inserted_at     | DATETIME   |

Constraint:

```sql
UNIQUE(repo_id, hash)
```

---

## commit_parents

Required for commit graph rendering.

| Column      | Type       |
| ----------- | ---------- |
| id          | INTEGER PK |
| repo_id     | INTEGER FK |
| commit_hash | TEXT       |
| parent_hash | TEXT       |

Example:

```text
A
|
B
|
C
```

Merge:

```text
A
|\
B C
 \|
  D
```

This table enables graph generation.

---

## commit_branches

Maps commits to visible branches.

| Column      | Type       |
| ----------- | ---------- |
| id          | INTEGER PK |
| repo_id     | INTEGER FK |
| commit_hash | TEXT       |
| branch_name | TEXT       |

---

# File Explorer Tables

## repository_files

Stores indexed file metadata.

| Column           | Type       |
| ---------------- | ---------- |
| id               | INTEGER PK |
| repo_id          | INTEGER FK |
| file_path        | TEXT       |
| file_name        | TEXT       |
| extension        | TEXT       |
| size_bytes       | INTEGER    |
| is_binary        | BOOLEAN    |
| last_modified_at | DATETIME   |

Indexes:

```sql
idx_repo_file_path
idx_repo_extension
```

---

## file_content_cache

Optional preview cache.

Used for:

* file previews
* search

| Column          | Type       |
| --------------- | ---------- |
| file_id         | INTEGER PK |
| preview_content | TEXT       |
| cached_at       | DATETIME   |

Limit:

First 50KB only.

---

# Diff System

## commit_file_stats

Existing table retained.

---

## diff_cache

Stores generated diffs.

Improves UI performance.

| Column       | Type       |
| ------------ | ---------- |
| id           | INTEGER PK |
| repo_id      | INTEGER FK |
| diff_key     | TEXT       |
| diff_type    | TEXT       |
| generated_at | DATETIME   |
| content      | TEXT       |

Examples:

commit_a:commit_b

branch_a:branch_b

working_tree

---

# Working Tree

## working_tree_snapshots

Existing table retained.

---

## working_tree_files

Tracks file-level working state.

| Column    | Type       |
| --------- | ---------- |
| id        | INTEGER PK |
| repo_id   | INTEGER FK |
| file_path | TEXT       |
| status    | TEXT       |

Values:

```text
MODIFIED
STAGED
UNTRACKED
DELETED
```

---

# Analytics Tables

## file_hotspots

Existing table retained.

---

## repo_health_metrics

Existing table retained.

---

## contributor_metrics

Precomputed contributor analytics.

| Column       | Type       |
| ------------ | ---------- |
| id           | INTEGER PK |
| repo_id      | INTEGER FK |
| author_email | TEXT       |
| commit_count | INTEGER    |
| churn        | INTEGER    |
| active_days  | INTEGER    |
| updated_at   | DATETIME   |

---

# Search Tables

## search_index

Local search acceleration.

| Column          | Type       |
| --------------- | ---------- |
| id              | INTEGER PK |
| repo_id         | INTEGER FK |
| entity_type     | TEXT       |
| entity_id       | TEXT       |
| searchable_text | TEXT       |

Entity types:

```text
REPOSITORY
FILE
COMMIT
BRANCH
```

---

# Indexing Engine Tables

## indexing_jobs

Tracks indexing work.

| Column        | Type       |
| ------------- | ---------- |
| id            | INTEGER PK |
| repo_id       | INTEGER FK |
| job_type      | TEXT       |
| status        | TEXT       |
| started_at    | DATETIME   |
| completed_at  | DATETIME   |
| error_message | TEXT       |

Statuses:

```text
PENDING
RUNNING
FAILED
COMPLETED
```

---

# Security Tables

## audit_logs

Critical security table.

| Column      | Type       |
| ----------- | ---------- |
| id          | INTEGER PK |
| action      | TEXT       |
| entity_type | TEXT       |
| entity_id   | TEXT       |
| details     | TEXT       |
| created_at  | DATETIME   |

Examples:

ROOT_ADDED

ROOT_REMOVED

SETTINGS_UPDATED

FUTURE_GIT_PULL

---

# Settings

## settings

Existing table retained.

---

# Future WakaTime Support

## wakatime_projects

| Column                | Type       |
| --------------------- | ---------- |
| id                    | INTEGER PK |
| repo_id               | INTEGER FK |
| wakatime_project_name | TEXT       |
| last_synced_at        | DATETIME   |

---

## wakatime_daily_stats

| Column              | Type       |
| ------------------- | ---------- |
| id                  | INTEGER PK |
| wakatime_project_id | INTEGER FK |
| date                | DATE       |
| coding_seconds      | INTEGER    |

---

# Recommended SQLite Optimizations

Enable:

```sql
PRAGMA foreign_keys = ON;

PRAGMA journal_mode = WAL;

PRAGMA synchronous = NORMAL;

PRAGMA temp_store = MEMORY;

PRAGMA cache_size = -20000;
```

---

# Migration Order

1. tracked_roots
2. tracked_repositories
3. branches
4. commits
5. commit_parents
6. commit_branches
7. repository_files
8. file_content_cache
9. commit_file_stats
10. diff_cache
11. working_tree_snapshots
12. working_tree_files
13. file_hotspots
14. contributor_metrics
15. repo_health_metrics
16. search_index
17. indexing_jobs
18. audit_logs
19. settings
20. wakatime_projects
21. wakatime_daily_stats

---

# Important Design Decision

Repository file contents are NOT fully stored in SQLite.

Only:

* Metadata
* Search previews
* Cached snippets

This prevents:

* Database bloat
* Privacy risks
* Slow indexing

Actual file content should always be read directly from disk when opened.
