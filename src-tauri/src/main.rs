// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod domain;
mod infrastructure;
mod security;
mod services;
mod state;

use infrastructure::database::{connection::get_connection, migrations::run_migrations};
use state::AppState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");

            let db_path = app_data_dir.join("gitradar.db");
            let conn = get_connection(&db_path).expect("failed to open application database");
            run_migrations(&conn).expect("failed to migrate application database");

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
            commands::repos::discover_repositories,
            commands::repos::add_tracked_root_path,
            commands::repos::get_all_tracked_root_paths,
            commands::repos::set_tracked_root_enabled,
            commands::repos::delete_tracked_root_path,
            // Branch commands
            commands::branches::get_repository_branches,
            commands::branches::get_branch_info,
            // Commit commands
            commands::commits::get_commits,
            commands::commits::get_commit_by_hash,
            commands::commits::get_commit_diff,
            commands::commits::get_file_diff,
            commands::commits::get_file_diff_history,
            // File commands
            commands::files::get_repository_files,
            commands::files::get_repository_file_by_path,
            commands::files::get_files_by_extension,
            commands::files::get_file_stats,
            commands::files::get_file_stats_by_path,
            commands::files::get_file_hotspots,
            commands::files::get_repo_languages_stats,
            // Analytics commands
            commands::analytics::get_repository_activity,
            // commands::analytics::get_contributors,
            // commands::analytics::get_top_contributors,
            // commands::analytics::get_contributor_by_email,
            // // Sync commands
            // commands::sync::calculate_repository_metrics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
