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
            commands::repos::get_repository_info,
            commands::repos::get_all_repositories,
            commands::repos::discover_repositories,
            commands::repos::add_tracked_root_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
