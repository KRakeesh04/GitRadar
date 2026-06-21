use git2::Error;
use std::path::Path;
use walkdir::WalkDir;

use crate::infrastructure::git::{DiscoveredRepository, RepositoryInfo, RepositoryType};

pub fn scan_repos_from_root(root_path: &Path) -> Result<Vec<DiscoveredRepository>, Error> {
    let mut repositories = Vec::new();
    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();
        let git_dir = path.join(".git");
        if !git_dir.exists() {
            continue;
        }

        let repo_type = if git_dir.is_file() {
            let content = std::fs::read_to_string(&git_dir).unwrap_or_default();
            if content.contains("/worktrees/") {
                RepositoryType::Worktree
            } else if content.contains("/modules/") {
                RepositoryType::Submodule
            } else {
                RepositoryType::Standard
            }
        } else {
            RepositoryType::Standard
        };

        repositories.push(DiscoveredRepository {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            path: path.to_path_buf(),
            git_dir,
            repo_type,
        });
    }
    Ok(repositories)
}

pub fn get_repository_info(repo_path: &str) -> Result<RepositoryInfo, String> {
    // Validate repository path exists
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    // Open the repository
    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    // Get remote URL
    let remote_url = repo
        .find_remote("origin")
        .ok()
        .and_then(|remote| remote.url().ok().map(String::from));

    // Get default branch
    let default_branch = repo
        .find_reference("HEAD")
        .ok()
        .and_then(|head| head.symbolic_target().ok().flatten().map(String::from));

    Ok(RepositoryInfo {
        remote_url,
        default_branch,
    })
}
