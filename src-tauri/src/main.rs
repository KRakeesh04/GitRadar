// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod domain;
mod infrastructure;
mod security;
mod services;
mod state;

use infrastructure::database::{connection::get_connection, migrations::run_migrations};
use services::search_index_service;
use state::AppState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");

            let db_path = app_data_dir.join("gitradar.db");
            let conn = get_connection(&db_path).expect("failed to open application database");
            run_migrations(&conn).expect("failed to migrate application database");

            // Rebuild the cross-entity searchable-text index in the background so
            // search works even if no full sync has run yet. Non-blocking.
            let rebuild_path = db_path.clone();
            std::thread::spawn(move || {
                if let Ok(mut conn) = get_connection(&rebuild_path) {
                    let _ = search_index_service::rebuild_search_index_from_db(&mut conn);
                }
            });

            app.manage(AppState {
                db_path,
                app_data_dir,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Repository commands
            commands::repos::get_repository_info,
            commands::repos::get_all_repositories,
            commands::repos::get_repositories_by_root_id,
            commands::repos::get_paginated_repositories,
            commands::repos::search_repositories,
            commands::repos::set_repository_enabled,
            commands::repos::discover_repositories,
            commands::repos::add_tracked_root_path,
            commands::repos::get_all_tracked_root_paths,
            commands::repos::set_tracked_root_enabled,
            commands::repos::delete_tracked_root_path,
            commands::repos::set_repository_starred,
            commands::repos::get_starred_repositories,
            commands::repos::get_recent_repositories,
            // Branch commands
            commands::branches::get_repository_branches,
            commands::branches::get_branch_info,
            // Commit commands
            commands::commits::get_commits,
            commands::commits::get_commit_by_hash,
            commands::commits::get_commit_graph,
            commands::commits::get_commit_diff,
            commands::commits::get_commit_inline_diff,
            commands::commits::get_file_diff,
            commands::commits::get_file_diff_history,
            // File commands
            commands::files::get_repository_files,
            commands::files::get_repository_file_tree,
            commands::files::get_repository_file_content,
            commands::files::get_repository_file_by_path,
            commands::files::get_files_by_extension,
            commands::files::get_file_stats,
            commands::files::get_file_stats_by_path,
            commands::files::get_file_hotspots,
            commands::files::get_repo_languages_stats,
            // Analytics commands
            commands::analytics::get_repository_activity,
            // Contributor commands
            commands::contributors::get_contributors,
            commands::contributors::get_top_contributors,
            commands::contributors::get_contributor_by_email,
            // Search commands
            commands::search::search_everything,
            commands::search::reindex_search_index,
            commands::search::rebuild_search_index,
            // Sync commands
            commands::sync::sync_repository,
            commands::sync::sync_branches,
            commands::sync::sync_commits,
            commands::sync::sync_contributors,
            commands::sync::sync_repository_files,
            commands::sync::sync_commit_file_stats,
            commands::sync::sync_repo_activity,
            commands::sync::sync_working_tree_status,
            commands::sync::sync_repository_health,
            commands::sync::sync_file_hotspots,
            commands::sync::get_indexing_jobs_by_repo,
            commands::sync::get_latest_indexing_job_by_repo,
            commands::sync::get_pending_indexing_jobs,
            commands::sync::cleanup_completed_indexing_jobs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
