# Service Changes

## Modified files

- `src-tauri/src/services/commit_service.rs`
  - `get_commits()` now maps database commits to `Commit` domain entities.
  - `get_commit_by_hash()` now maps the stored commit hash, body, and optional author fields correctly.
  - `map_commit()` and `commit_database_error()` centralize conversion and error handling.
- `src-tauri/src/services/file_service.rs`
  - `get_repository_files()` returns all persisted repository-file domain entities.
  - `get_repository_file_by_path()` returns one persisted file or a not-found domain error.
  - `get_files_by_extension()` returns persisted files filtered by extension.
  - `get_file_stats()` and `get_file_stats_by_path()` return stored file-change statistics.
  - `get_file_hotspots()` returns stored hotspot values without recalculation.
- `src-tauri/src/services/analytics_service.rs`
  - `get_repository_activity()` returns persisted daily activity rows.
  - `get_contributors()`, `get_top_contributors()`, and `get_contributor_by_email()` return persisted contributor domain entities.
  - Mapping helpers centralize database-to-domain conversion and errors.
- `src-tauri/src/services/mod.rs`
  - Exposes the analytics and file services.
- `src-tauri/src/domain/file.rs`
  - Adds data-only domain entities required to represent persisted file, hotspot, file-stat, and daily-activity records.
- `src-tauri/src/domain/mod.rs`
  - Exposes the new data-only domain entities.
- `src-tauri/src/commands/repos.rs`
  - Routes existing repository commands to the already separated query and discovery services so the crate compiles.

## Assumptions

- File hotspots and activity rows are precomputed and persisted; the services only read and map them.
- Missing optional commit identity fields map to empty strings because the current `Commit` domain entity requires strings.
- Negative persisted counters are invalid data and return an existing `DomainError` instead of being silently converted.
- File-related failures use `InvalidRepository`, and contributor conversion failures use `InvalidCommit`, because those are the applicable existing error variants.

## Usage

```rust
let commits = commit_service::get_commits(&conn, repo_id, 50, 0)?;
let commit = commit_service::get_commit_by_hash(&conn, repo_id, hash)?;
```

```rust
let files = file_service::get_repository_files(&conn, repo_id)?;
let hotspots = file_service::get_file_hotspots(&conn, repo_id)?;
```

```rust
let activity = analytics_service::get_repository_activity(&conn, repo_id, None, None)?;
let contributors = analytics_service::get_top_contributors(&conn, repo_id, Some(10))?;
```
