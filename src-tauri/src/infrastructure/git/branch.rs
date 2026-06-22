use crate::infrastructure::git::NewBranch;

pub fn get_branches(repo_path: &str) -> Result<Vec<NewBranch>, String> {
    // Validate repository path exists
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    // Open the repository
    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    // Get the list of branches
    let branches = match repo.branches(None) {
        Ok(branches) => branches,
        Err(e) => return Err(format!("Failed to get branches: {}", e)),
    };

    let mut branch_list = Vec::new();
    for branch in branches {
        match branch {
            Ok((branch, _)) => {
                if let Some(name) = branch.name().unwrap_or(None) {
                    branch_list.push(NewBranch {
                        name: name.to_string(),
                        is_head: branch.is_head(),
                    });
                }
            }
            Err(e) => return Err(format!("Failed to read branch: {}", e)),
        }
    }

    Ok(branch_list)
}

pub fn is_local_branch(repo_path: &str, branch_name: &str) -> Result<bool, String> {
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    // Check if the branch exists locally
    let branch_exists = match repo.find_branch(branch_name, git2::BranchType::Local) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.code() == git2::ErrorCode::NotFound {
                Ok(false)
            } else {
                Err(format!("Failed to check branch '{}': {}", branch_name, e))
            }
        }
    };

    branch_exists
}

pub fn is_remote_branch(repo_path: &str, branch_name: &str) -> Result<bool, String> {
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    // Check if the branch exists remotely
    let branch_exists = match repo.find_branch(branch_name, git2::BranchType::Remote) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.code() == git2::ErrorCode::NotFound {
                Ok(false)
            } else {
                Err(format!("Failed to check branch '{}': {}", branch_name, e))
            }
        }
    };

    branch_exists
}

pub fn current_head_branch(repo_path: &str) -> Result<String, String> {
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => return Err(format!("Failed to get HEAD reference: {}", e)),
    };

    let branch_name = head.shorthand().unwrap_or("").to_string();

    Ok(branch_name)
}
