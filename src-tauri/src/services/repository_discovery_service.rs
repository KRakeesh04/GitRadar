use std::path::PathBuf;

use rayon::prelude::*;
use rusqlite::Connection;

use crate::infrastructure::{
    database::repositories::{repositories, tracked_roots},
    git::{branch, repo},
};

pub fn discover_repositories(conn: &mut Connection) -> Result<(), String> {
    let roots = tracked_roots::get_all_tracked_roots(conn)
        .map_err(|e| format!("Failed to load tracked roots: {}", e))?;

    let discovered: Vec<_> = roots
        .into_par_iter()
        .filter(|root| root.is_enabled)
        .flat_map(
            |root| match repo::scan_repos_from_root(&PathBuf::from(&root.path)) {
                Ok(repositories) => repositories
                    .into_iter()
                    .map(|repo| (root.id, repo))
                    .collect::<Vec<_>>(),

                Err(error) => {
                    eprintln!("Failed to scan '{}': {}", root.path, error);
                    Vec::new()
                }
            },
        )
        .collect();

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    for (root_id, repo) in discovered {
        let repo_path = match repo.path.to_str() {
            Some(path) => path,
            None => continue,
        };

        let head_branch = branch::current_head_branch(repo_path).ok();

        let repo_info = match repo::get_repository_info(repo_path) {
            Ok(info) => info,
            Err(error) => {
                eprintln!("Failed to read repository info '{}': {}", repo_path, error);
                continue;
            }
        };

        repositories::upsert_repository(
            &tx,
            root_id,
            &repo.name,
            repo_path,
            repo.git_dir.to_str().unwrap_or(""),
            &format!("{:?}", repo.repo_type),
            repo_info.remote_url.as_deref(),
            repo_info.default_branch.as_deref(),
            head_branch.as_deref(),
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}
