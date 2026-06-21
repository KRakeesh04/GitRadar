use crate::infrastructure::git::WorkingTreeStatus;

pub fn get_working_tree_statuses(repo_path: &str) -> Result<WorkingTreeStatus, String> {
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    let repo = git2::Repository::open(repo_path)
        .map_err(|error| format!("Failed to open repository: {error}"))?;

    let mut options = git2::StatusOptions::new();
    options
        .include_untracked(true)
        .renames_index_to_workdir(true);

    let statuses = repo
        .statuses(Some(&mut options))
        .map_err(|error| format!("Failed to get repository status: {error}"))?;

    let mut working_tree_status = WorkingTreeStatus {
        added: Vec::new(),
        modified: Vec::new(),
        deleted: Vec::new(),
        renamed: Vec::new(),
    };

    for entry in statuses.iter() {
        let status = entry.status();

        if status.contains(git2::Status::WT_RENAMED) {
            let delta = entry.index_to_workdir().ok_or_else(|| {
                "Git reported a working-tree rename without rename details".to_string()
            })?;
            let old_path = delta
                .old_file()
                .path()
                .ok_or_else(|| "Git reported a rename without an old path".to_string())?
                .to_string_lossy()
                .into_owned();
            let new_path = delta
                .new_file()
                .path()
                .ok_or_else(|| "Git reported a rename without a new path".to_string())?
                .to_string_lossy()
                .into_owned();

            working_tree_status.renamed.push((old_path, new_path));
        } else {
            let path = entry
                .path()
                .map_err(|error| format!("Failed to read status path: {error}"))?
                .to_string();

            if status.contains(git2::Status::WT_NEW) {
                working_tree_status.added.push(path);
            } else if status.contains(git2::Status::WT_DELETED) {
                working_tree_status.deleted.push(path);
            } else if status.contains(git2::Status::WT_MODIFIED) {
                working_tree_status.modified.push(path);
            }
        }
    }

    Ok(working_tree_status)
}
