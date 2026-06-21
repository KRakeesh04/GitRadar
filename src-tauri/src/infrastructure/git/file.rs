use crate::infrastructure::git::FilePatch;

pub fn get_file_patch(
    repo_path: &str,
    commit_hash: &str,
    file_path: &str,
) -> Result<FilePatch, String> {
    let repo = git2::Repository::open(repo_path).map_err(|e| e.to_string())?;

    let oid = git2::Oid::from_str(commit_hash).map_err(|e| e.to_string())?;

    let commit = repo.find_commit(oid).map_err(|e| e.to_string())?;

    if commit.parent_count() == 0 {
        return Ok(FilePatch {
            file_path: file_path.to_string(),
            old_path: None,
            change_type: "ADDED".to_string(),
            patch: String::new(),
        });
    }

    let parent = commit.parent(0).map_err(|e| e.to_string())?;

    let parent_tree = parent.tree().map_err(|e| e.to_string())?;

    let current_tree = commit.tree().map_err(|e| e.to_string())?;

    let diff = repo
        .diff_tree_to_tree(Some(&parent_tree), Some(&current_tree), None)
        .map_err(|e| e.to_string())?;

    let mut patch_text = String::new();
    let mut change_type = "MODIFIED".to_string();
    let mut old_path = None;

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        let current_path = delta.new_file().path().or(delta.old_file().path());

        if let Some(path) = current_path {
            if path.to_string_lossy() == file_path {
                patch_text.push_str(std::str::from_utf8(line.content()).unwrap_or(""));

                change_type = format!("{:?}", delta.status());

                old_path = delta
                    .old_file()
                    .path()
                    .map(|p| p.to_string_lossy().to_string());
            }
        }

        true
    })
    .map_err(|e| e.to_string())?;

    Ok(FilePatch {
        file_path: file_path.to_string(),
        old_path,
        change_type,
        patch: patch_text,
    })
}

pub fn get_commit_patch(repo_path: &str, commit_hash: &str) -> Result<Vec<FilePatch>, String> {
    let repo = git2::Repository::open(repo_path).map_err(|e| e.to_string())?;

    let oid = git2::Oid::from_str(commit_hash).map_err(|e| e.to_string())?;

    let commit = repo.find_commit(oid).map_err(|e| e.to_string())?;

    if commit.parent_count() == 0 {
        return Ok(Vec::new());
    }

    let parent = commit.parent(0).map_err(|e| e.to_string())?;

    let parent_tree = parent.tree().map_err(|e| e.to_string())?;

    let current_tree = commit.tree().map_err(|e| e.to_string())?;

    let diff = repo
        .diff_tree_to_tree(Some(&parent_tree), Some(&current_tree), None)
        .map_err(|e| e.to_string())?;

    let mut patches = Vec::new();

    for delta in diff.deltas() {
        let file_path = delta
            .new_file()
            .path()
            .or(delta.old_file().path())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let old_path = delta
            .old_file()
            .path()
            .map(|p| p.to_string_lossy().to_string());

        let change_type = format!("{:?}", delta.status());

        let patch = get_file_patch(repo_path, commit_hash, &file_path)?;

        patches.push(FilePatch {
            file_path,
            old_path,
            change_type,
            patch: patch.patch,
        });
    }

    Ok(patches)
}
