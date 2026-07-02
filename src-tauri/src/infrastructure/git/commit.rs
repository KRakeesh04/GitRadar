use std::collections::HashMap;

use crate::infrastructure::git::{change_type, FileHotspotPerCommit, LastCommit, NewCommit};

pub fn head_commit_hash(repo_path: &str) -> Result<String, String> {
    // Validate repository path exists
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    // Open the repository
    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    // Get the HEAD reference
    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => return Err(format!("Failed to get HEAD reference: {}", e)),
    };

    // Get the commit hash from HEAD
    let head_commit_hash = match head.target() {
        Some(hash) => hash.to_string(),
        None => return Err("HEAD does not point to a valid commit".to_string()),
    };

    Ok(head_commit_hash)
}

pub fn commit_info(repo_path: &str, commit_hash: &str) -> Result<NewCommit, String> {
    // Validate repository path exists
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    // Open the repository
    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    // Convert commit hash string to Oid
    let oid = match git2::Oid::from_str(commit_hash) {
        Ok(oid) => oid,
        Err(e) => return Err(format!("Invalid commit hash '{}': {}", commit_hash, e)),
    };

    // Get the commit object
    let commit = match repo.find_commit(oid) {
        Ok(commit) => commit,
        Err(e) => return Err(format!("Failed to find commit '{}': {}", commit_hash, e)),
    };

    let hash = commit_hash.to_string();
    let author_name = commit.author().name().ok().map(String::from);
    let author_email = commit.author().email().ok().map(String::from);
    let committer_name = commit.committer().name().ok().map(String::from);
    let committer_email = commit.committer().email().ok().map(String::from);
    let subject = commit
        .summary()
        .ok()
        .flatten()
        .unwrap_or("No subject")
        .to_string();
    let body = commit.message().ok().map(String::from);
    let parent_count = commit.parent_count() as i32;
    let committed_at = commit.time().seconds().to_string();

    let new_commit = NewCommit {
        hash,
        author_name,
        author_email,
        committer_name,
        committer_email,
        subject,
        body,
        parent_count,
        committed_at,
    };

    Ok(new_commit)
}

pub fn last_commit_info_by_branch(
    repo_path: &str,
    branch_name: &str,
) -> Result<LastCommit, String> {
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    let branch_ref = match repo.find_branch(branch_name, git2::BranchType::Local) {
        Ok(branch) => branch.into_reference(),
        Err(e) => return Err(format!("Failed to find branch '{}': {}", branch_name, e)),
    };

    let commit_hash = match branch_ref.target() {
        Some(hash) => hash.to_string(),
        None => {
            return Err(format!(
                "Branch '{}' does not point to a valid commit",
                branch_name
            ))
        }
    };

    // Get the commit info using the existing function
    commit_info(repo_path, &commit_hash).map(|new_commit| LastCommit {
        hash: new_commit.hash,
        committed_at: new_commit.committed_at,
    })
}

pub fn find_ahead_behind_local_vs_remote(
    repo_path: &str,
    branch: &str,
) -> Result<(i32, i32), String> {
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    let remote_ref = match repo.find_branch(branch, git2::BranchType::Remote) {
        Ok(branch) => branch.into_reference(),
        Err(e) => return Err(format!("Failed to find remote branch '{}': {}", branch, e)),
    };

    let local_ref = match repo.find_branch(branch, git2::BranchType::Local) {
        Ok(branch) => branch.into_reference(),
        Err(_) => remote_ref.clone(), // If local branch doesn't exist, use remote ref for comparison
    };

    let (ahead, behind) =
        match repo.graph_ahead_behind(local_ref.target().unwrap(), remote_ref.target().unwrap()) {
            Ok((ahead, behind)) => (ahead as i32, behind as i32),
            Err(e) => return Err(format!("Failed to calculate ahead/behind: {}", e)),
        };

    Ok((ahead, behind))
}

pub fn find_ahead_behind_given_vs_default(
    repo_path: &str,
    default_branch: &str,
    given_branch: &str,
) -> Result<(i32, i32), String> {
    if default_branch == given_branch {
        return Ok((0, 0));
    }

    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    let given_ref = match repo.find_branch(given_branch, git2::BranchType::Local) {
        Ok(branch) => branch.into_reference(),
        Err(e) => {
            return Err(format!(
                "Failed to find given branch '{}': {}",
                given_branch, e
            ))
        }
    };

    let default_ref = match repo.find_branch(default_branch, git2::BranchType::Local) {
        Ok(branch) => branch.into_reference(),
        Err(e) => {
            return Err(format!(
                "Failed to find default branch '{}': {}",
                default_branch, e
            ))
        }
    };

    let (ahead, behind) =
        match repo.graph_ahead_behind(given_ref.target().unwrap(), default_ref.target().unwrap()) {
            Ok((ahead, behind)) => (ahead as i32, behind as i32),
            Err(e) => return Err(format!("Failed to calculate ahead/behind: {}", e)),
        };

    Ok((ahead, behind))
}

pub fn get_file_hotspot_per_commit_hash(
    repo_path: &str,
    commit_hash: &str,
) -> Result<Vec<FileHotspotPerCommit>, String> {
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    let oid = match git2::Oid::from_str(commit_hash) {
        Ok(oid) => oid,
        Err(e) => return Err(format!("Invalid commit hash '{}': {}", commit_hash, e)),
    };

    let commit = match repo.find_commit(oid) {
        Ok(commit) => commit,
        Err(e) => return Err(format!("Failed to find commit '{}': {}", commit_hash, e)),
    };

    let commit_tree = commit
        .tree()
        .map_err(|error| format!("Failed to read commit tree: {error}"))?;
    let parent_tree = if commit.parent_count() == 0 {
        None
    } else {
        Some(
            commit
                .parent(0)
                .and_then(|parent| parent.tree())
                .map_err(|error| format!("Failed to read parent tree: {error}"))?,
        )
    };
    let mut diff = repo
        .diff_tree_to_tree(parent_tree.as_ref(), Some(&commit_tree), None)
        .map_err(|error| format!("Failed to get commit diff: {error}"))?;
    diff.find_similar(None)
        .map_err(|error| format!("Failed to detect renamed files: {error}"))?;

    let mut file_hotspots = Vec::with_capacity(diff.deltas().len());
    let mut hotspot_indices = HashMap::new();

    for delta in diff.deltas() {
        let file_path = delta
            .new_file()
            .path()
            .or(delta.old_file().path())
            .map(|path| path.to_string_lossy().into_owned());
        let Some(file_path) = file_path else {
            continue;
        };

        hotspot_indices.insert(delta_key(&delta), file_hotspots.len());
        file_hotspots.push(FileHotspotPerCommit {
            file_path,
            addions: 0,
            deletions: 0,
            change_type: change_type(delta.status()),
        });
    }

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(&index) = hotspot_indices.get(&delta_key(&delta)) {
            match line.origin() {
                '+' => file_hotspots[index].addions += 1,
                '-' => file_hotspots[index].deletions += 1,
                _ => {}
            }
        }
        true
    })
    .map_err(|error| format!("Failed to calculate line changes: {error}"))?;

    Ok(file_hotspots)
}

fn delta_key(delta: &git2::DiffDelta<'_>) -> String {
    format!(
        "{:?}\0{}\0{}",
        delta.status(),
        delta
            .old_file()
            .path()
            .map(|path| path.to_string_lossy())
            .unwrap_or_default(),
        delta
            .new_file()
            .path()
            .map(|path| path.to_string_lossy())
            .unwrap_or_default(),
    )
}
