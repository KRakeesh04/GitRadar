use super::DomainResult;

#[derive(Debug, Clone)]
pub struct Commit {
    // Identity
    pub id: CommitId,
    pub hash: CommitHash,

    // Author Information
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,

    // Message
    pub subject: String,
    pub body: Option<String>,

    // Metadata
    pub parent_count: u32,
    pub committed_at: String, // ISO 8601 timestamp

    // Business State
    pub is_significant: bool, // Large commit worth highlighting
}

impl Commit {
    // Create a new commit domain entity
    pub fn new(
        id: i64,
        hash: String,
        author_name: String,
        author_email: String,
        committer_name: String,
        committer_email: String,
        subject: String,
        parent_count: u32,
        committed_at: String,
    ) -> DomainResult<Self> {
        if hash.is_empty() {
            return Err(super::DomainError::InvalidCommit(
                "Commit hash cannot be empty".to_string(),
            ));
        }

        if subject.is_empty() {
            return Err(super::DomainError::InvalidCommit(
                "Commit subject cannot be empty".to_string(),
            ));
        }

        Ok(Commit {
            id: CommitId(id),
            hash: CommitHash(hash),
            author_name,
            author_email,
            committer_name,
            committer_email,
            subject,
            body: None,
            parent_count,
            committed_at,
            is_significant: false,
        })
    }

    // has 2+ parents
    pub fn is_merge_commit(&self) -> bool {
        self.parent_count >= 2
    }

    pub fn is_root_commit(&self) -> bool {
        self.parent_count == 0
    }

    // exactly 1 parent
    pub fn is_regular_commit(&self) -> bool {
        self.parent_count == 1
    }

    pub fn commit_type(&self) -> CommitType {
        match self.parent_count {
            0 => CommitType::Root,
            1 => CommitType::Regular,
            _ => CommitType::Merge,
        }
    }

    // Get commit message size (subject + body length)
    pub fn message_size(&self) -> usize {
        let subject_len = self.subject.len();
        let body_len = self.body.as_ref().map(|b| b.len()).unwrap_or(0);
        subject_len + body_len
    }

    // has body + subject > 10 chars
    pub fn is_well_documented(&self) -> bool {
        self.body.is_some() && self.subject.len() > 10
    }

    // Significant = merge commit OR well-documented regular commit
    pub fn determine_significance(&mut self) {
        self.is_significant = self.is_merge_commit() || self.is_well_documented();
    }

    // Get first 50 chars of subject in commit message, for display purposes
    pub fn short_message(&self) -> String {
        let mut short = self.subject.chars().take(50).collect::<String>();
        if self.subject.len() > 50 {
            short.push_str("...");
        }
        short
    }

    pub fn get_commit_info(&self) -> CommitInfo {
        CommitInfo {
            hash: self.hash.short(),
            author: self.author_name.clone(),
            subject: self.short_message(),
            commit_type: self.commit_type(),
            is_significant: self.is_significant,
            timestamp: self.committed_at.clone(),
        }
    }

    // Validate commit integrity
    pub fn validate(&self) -> DomainResult<()> {
        if self.hash.0.len() != 40 && self.hash.0.len() != 7 {
            return Err(super::DomainError::InvalidCommit(format!(
                "Invalid commit hash length: {}",
                self.hash.0.len()
            )));
        }

        if self.author_name.is_empty() {
            return Err(super::DomainError::InvalidCommit(
                "Author name cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    pub fn set_body(&mut self, body: String) {
        self.body = if body.is_empty() { None } else { Some(body) };
    }
}

// Commit ID - Unique identifier in database
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommitId(pub i64);

// Commit Hash - The git commit SHA
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitHash(pub String);

impl CommitHash {
    // Get short hash (first 7 characters)
    pub fn short(&self) -> String {
        self.0.chars().take(7).collect()
    }

    // Get full hash
    pub fn full(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitType {
    Root,    // Initial commit (no parents)
    Regular, // Normal commit (1 parent)
    Merge,   // Merge commit (2+ parents)
}

impl std::fmt::Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitType::Root => write!(f, "Root"),
            CommitType::Regular => write!(f, "Regular"),
            CommitType::Merge => write!(f, "Merge"),
        }
    }
}

// CommitInfo - Simplified commit information for display
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub subject: String,
    pub commit_type: CommitType,
    pub is_significant: bool,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub struct CommitGraphNode {
    pub hash: String,
    pub branches: Vec<String>,
    pub author_name: String,
    pub author_email: String,
    pub subject: String,
    pub committed_at: String,
    pub total_additions: i32,
    pub total_deletions: i32,
    pub total_files_changed: i32,
    pub parents: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_creation() {
        let commit = Commit::new(
            1,
            "abc123def456".to_string(),
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "Fix: login issue".to_string(),
            1,
            "2024-01-01T10:00:00Z".to_string(),
        )
        .unwrap();

        assert_eq!(commit.author_name, "John Doe");
        assert_eq!(commit.subject, "Fix: login issue");
    }

    #[test]
    fn test_commit_types() {
        let root = Commit::new(
            1,
            "abc123def456".to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
            "Initial commit".to_string(),
            0,
            "2024-01-01T10:00:00Z".to_string(),
        )
        .unwrap();

        let regular = Commit::new(
            2,
            "def456abc789".to_string(),
            "Jane".to_string(),
            "jane@example.com".to_string(),
            "Jane".to_string(),
            "jane@example.com".to_string(),
            "Add feature".to_string(),
            1,
            "2024-01-02T10:00:00Z".to_string(),
        )
        .unwrap();

        let merge = Commit::new(
            3,
            "789abc123def".to_string(),
            "Bob".to_string(),
            "bob@example.com".to_string(),
            "Bob".to_string(),
            "bob@example.com".to_string(),
            "Merge branch feature".to_string(),
            2,
            "2024-01-03T10:00:00Z".to_string(),
        )
        .unwrap();

        assert!(root.is_root_commit());
        assert!(regular.is_regular_commit());
        assert!(merge.is_merge_commit());
    }

    #[test]
    fn test_commit_type_enum() {
        let root = Commit::new(
            1,
            "abc123".to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
            "Initial".to_string(),
            0,
            "2024-01-01T10:00:00Z".to_string(),
        )
        .unwrap();

        assert_eq!(root.commit_type(), CommitType::Root);
    }

    #[test]
    fn test_short_message() {
        let commit = Commit::new(
            1,
            "abc123".to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
            "This is a very long commit message that should be truncated when displayed in short form".to_string(),
            1,
            "2024-01-01T10:00:00Z".to_string(),
        )
        .unwrap();

        let short = commit.short_message();
        assert!(short.len() <= 53); // 50 + "..."
    }

    #[test]
    fn test_commit_significance() {
        let mut regular = Commit::new(
            1,
            "abc123".to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
            "Short msg".to_string(),
            1,
            "2024-01-01T10:00:00Z".to_string(),
        )
        .unwrap();

        regular.determine_significance();
        assert!(!regular.is_significant); // Short message, not merge

        let mut merge = Commit::new(
            2,
            "def456".to_string(),
            "Jane".to_string(),
            "jane@example.com".to_string(),
            "Jane".to_string(),
            "jane@example.com".to_string(),
            "Merge branch".to_string(),
            2,
            "2024-01-02T10:00:00Z".to_string(),
        )
        .unwrap();

        merge.determine_significance();
        assert!(merge.is_significant); // Merge commit
    }

    #[test]
    fn test_commit_hash_short() {
        let hash = CommitHash("abc123def456abc123def456abc123def456abc1".to_string());
        assert_eq!(hash.short(), "abc123d");
        assert_eq!(hash.full(), "abc123def456abc123def456abc123def456abc1");
    }
}
