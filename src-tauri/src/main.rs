// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod db;
mod models;
mod state;

use state::AppState;
use std::path::PathBuf;
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

            app.manage(AppState {
                db_path,
                app_data_dir,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::repos::get_all_repositories,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}