use crate::infrastructure::git::GraphNode;

pub fn get_graph_nodes(
    repo_path: &str,
    node_count: usize,
    skip: usize,
) -> Result<Vec<GraphNode>, String> {
    if !std::path::Path::new(repo_path).exists() {
        return Err(format!("Repository path '{}' does not exist", repo_path));
    }

    let repo = git2::Repository::open(repo_path)
        .map_err(|error| format!("Failed to open repository: {error}"))?;

    let mut revwalk = repo
        .revwalk()
        .map_err(|error| format!("Failed to create revwalk: {error}"))?;
    revwalk
        .push_head()
        .map_err(|error| format!("Failed to push HEAD to revwalk: {error}"))?;
    revwalk
        .set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)
        .map_err(|error| format!("Failed to set revwalk sorting: {error}"))?;

    let mut nodes = Vec::new();
    for (i, oid_result) in revwalk.enumerate() {
        if i < skip {
            continue; // Skip the specified number of commits
        }
        if nodes.len() >= node_count {
            break; // Stop after collecting the specified number of nodes
        }

        let oid = oid_result.map_err(|error| format!("Failed to get OID from revwalk: {error}"))?;
        let commit = repo
            .find_commit(oid)
            .map_err(|error| format!("Failed to find commit for OID {}: {error}", oid))?;

        nodes.push(GraphNode {
            hash: commit.id().to_string(),
            message: commit.summary().unwrap_or("").to_string(),
            author_name: commit.author().name().unwrap_or("").to_string(),
            author_email: commit.author().email().unwrap_or("").to_string(),
            committed_at: commit.time().seconds(),
            parent_hashes: commit
                .parent_ids()
                .map(|parent_oid| parent_oid.to_string())
                .collect(),
        });
    }

    Ok(nodes)
}
