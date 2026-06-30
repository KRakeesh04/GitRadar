use rusqlite::Connection;

use crate::{
    domain::{commit::CommitGraphNode, Commit, DomainError, DomainResult},
    infrastructure::database::{
        models::commit::Commit as DatabaseCommit,
        models::commit::CommitGraphNode as DatabaseCommitGraphNode, repositories::commits,
    },
};

pub fn get_commits(
    conn: &Connection,
    repo_id: i64,
    count: usize,
    offset: usize,
) -> DomainResult<Vec<Commit>> {
    let commits = match commits::get_commits_by_repo(conn, repo_id, count, offset) {
        Ok(Some(commits)) => commits,
        Ok(None) => return Ok(Vec::new()),
        Err(error) => return Err(commit_database_error("load commits", error)),
    };

    commits.into_iter().map(map_commit).collect()
}

pub fn get_commit_by_hash(conn: &Connection, repo_id: i64, hash: &str) -> DomainResult<Commit> {
    let commit = match commits::get_commit_by_hash(conn, repo_id, hash) {
        Ok(Some(commit)) => commit,
        Ok(None) => return Err(DomainError::InvalidCommit("Commit not found".into())),
        Err(error) => return Err(commit_database_error("load commit", error)),
    };

    map_commit(commit)
}

pub fn get_commit_graph(
    conn: &Connection,
    repo_id: i64,
    limit: usize,
    offset: usize,
) -> DomainResult<Vec<CommitGraphNode>> {
    let commit_graph = match commits::get_commit_graph(conn, repo_id, limit, offset) {
        Ok(commit_graph) => commit_graph,
        Err(error) => return Err(commit_database_error("load commit graph", error)),
    };

    commit_graph
        .into_iter()
        .map(map_commit_graph_node)
        .collect()
}

// TODO: pub fn get_commit_diff() to show all files changed in a commit with the diff of each file

// TODO: pub fn get_file_patch() to show the full version history of a file with commit hashes

fn map_commit(commit: DatabaseCommit) -> DomainResult<Commit> {
    let parent_count = u32::try_from(commit.parent_count).map_err(|_| {
        DomainError::InvalidCommit(format!(
            "Commit '{}' has a negative parent count",
            commit.hash
        ))
    })?;

    let mut domain_commit = Commit::new(
        commit.id,
        commit.hash,
        commit.author_name.unwrap_or_default(),
        commit.author_email.unwrap_or_default(),
        commit.committer_name.unwrap_or_default(),
        commit.committer_email.unwrap_or_default(),
        commit.subject,
        parent_count,
        commit.committed_at,
    )?;
    if let Some(body) = commit.body {
        domain_commit.set_body(body);
    }

    Ok(domain_commit)
}

fn map_commit_graph_node(node: DatabaseCommitGraphNode) -> DomainResult<CommitGraphNode> {
    Ok(CommitGraphNode {
        hash: node.hash,
        branches: node.branch_names,
        author_name: node.author_name,
        author_email: node.author_email,
        subject: node.subject,
        committed_at: node.committed_at,
        total_additions: node.additions,
        total_deletions: node.deletions,
        total_files_changed: node.total_changed_files_count,
        parents: node.parent_hashes,
    })
}

fn commit_database_error(action: &str, error: rusqlite::Error) -> DomainError {
    DomainError::InvalidCommit(format!("Failed to {action}: {error}"))
}
