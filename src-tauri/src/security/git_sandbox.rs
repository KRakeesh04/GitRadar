pub enum GitCommand {
    Status,
    Log,
    Show,
    Diff,
    Branch,
    // Future: Pull, Push, Merge (with additional security)
}
 
pub fn execute_git_command(repo_path: &Path, command: GitCommand) -> Result<String> {
    match command {
        GitCommand::Status => git_status(repo_path),
        GitCommand::Log => git_log(repo_path),
    }
}