use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Repo {
    pub name: String,
    pub path: String,
}

pub fn scan_repositories(paths_list: Vec<String>) -> Result<Vec<Repo>, String> {
    let mut repos = Vec::new();

    for path in paths_list {
        let path = PathBuf::from(path);
        if !path.exists() {
            continue;
        }
        visit_dirs(&path, &mut repos);
    }
    
    Ok(repos)
}

fn visit_dirs(dir: &Path, repos: &mut Vec<Repo>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.is_dir() {
            let git_path = path.join(".git");
            if git_path.exists() && git_path.is_dir() {
                let repo_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if !repos.iter().any(|r| r.path == path.to_string_lossy().to_string()) {
                    repos.push(Repo {
                        name: repo_name,
                        path: path.to_string_lossy().to_string(),
                    });
                }
                continue;
            }
            visit_dirs(&path, repos);
        }
    }
}