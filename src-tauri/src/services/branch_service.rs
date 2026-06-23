use rusqlite::Connection;

use crate::{
    domain::{branch::BranchId, Branch, BranchType, DomainError, DomainResult},
    infrastructure::database::repositories::branches,
};

pub fn get_branch_info_by_name(
    conn: &Connection,
    repo_id: i64,
    name: &str,
) -> DomainResult<Branch> {
    let branch = match branches::get_branch_by_name(conn, repo_id, name) {
        Ok(Some(branch)) => branch,
        Ok(None) => return Err(DomainError::InvalidBranch("Branch not found".into())),
        Err(e) => {
            return Err(DomainError::InvalidBranch(format!(
                "Failed to load branch: {}",
                e
            )))
        }
    };

    let branch_type = BranchType::from_name(&branch.name);

    Ok(Branch {
        id: BranchId(branch.id),
        repo_id: branch.repo_id,
        name: branch.name,
        is_default: branch.is_default,
        is_head: branch.is_head,
        branch_type,
        last_commit_hash: branch.last_commit_hash,
        ahead_count_from_remote: branch.ahead_count_from_remote as u32,
        behind_count_from_remote: branch.behind_count_from_remote as u32,
        ahead_count_from_default: branch.ahead_count_from_default as u32,
        behind_count_from_default: branch.behind_count_from_default as u32,
    })
}

pub fn get_repository_branches(conn: &Connection, repo_id: i64) -> DomainResult<Vec<Branch>> {
    let branches = match branches::get_all_branches(conn, repo_id) {
        Ok(Some(branches)) => branches,
        Ok(None) => return Ok(Vec::new()),
        Err(e) => {
            return Err(DomainError::InvalidBranch(format!(
                "Failed to load branches: {}",
                e
            )))
        }
    };

    let result = branches
        .into_iter()
        .map(|branch| {
            let branch_type = BranchType::from_name(&branch.name);
            Branch {
                id: BranchId(branch.id),
                repo_id: branch.repo_id,
                name: branch.name,
                is_default: branch.is_default,
                is_head: branch.is_head,
                branch_type,
                last_commit_hash: branch.last_commit_hash,
                ahead_count_from_remote: branch.ahead_count_from_remote as u32,
                behind_count_from_remote: branch.behind_count_from_remote as u32,
                ahead_count_from_default: branch.ahead_count_from_default as u32,
                behind_count_from_default: branch.behind_count_from_default as u32,
            }
        })
        .collect();

    Ok(result)
}
