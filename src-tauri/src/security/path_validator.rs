pub fn validate_path(path: &str, approved_roots: &[String]) -> Result<PathBuf, SecurityError> {
    let canonical = PathBuf::from(path).canonicalize()?;
    for root in approved_roots {
        if canonical.starts_with(root) {
            return Ok(canonical);
        }
    }
    Err(SecurityError::PathOutsideApprovedRoot)
}