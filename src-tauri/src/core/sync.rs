use crate::db::connection::get_connection;
use crate::db::repositories::{get_all_repositories as get_repos_from_db, insert_repository};
use crate::models::Repository;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub struct RepositorySync {
    pub repo_path: PathBuf,
}

impl RepositorySync {
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    pub fn start_background_sync(&self) {
        let repo_path = self.repo_path.clone();

        thread::spawn(move || {
            loop {
                if let Err(e) = Self::sync_repository(&repo_path) {
                    eprintln!("Sync error: {}", e);
                }

                // Sync every 30 seconds
                thread::sleep(Duration::from_secs(30));
            }
        });
    }

    fn sync_repository(repo_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let base_dir = dirs::data_local_dir().ok_or("failed to resolve local data directory")?;
        let db_path = base_dir
            .join("com.tauri.dev")
            .join("GitRadar")
            .join("gitradar.db");
        let conn = get_connection(&db_path)?;

        // Check if repository already exists
        let existing_repos = get_repos_from_db(&conn)?;
        let repo_path_str = repo_path.to_string_lossy();

        let repo_exists = existing_repos.iter().any(|r| r.path == repo_path_str);

        if !repo_exists {
            // Insert new repository
            let repo_name = repo_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            let git_dir_path = repo_path.join(".git").to_string_lossy().to_string();

            insert_repository(
                &conn,
                1, // root_id (you'd get this from tracked_roots table)
                &repo_name,
                &repo_path_str,
                &git_dir_path,
                "", // remote_url - empty for now, could be detected later
            )?;

            println!("Synced new repository: {}", repo_name);
        }

        Ok(())
    }
}
