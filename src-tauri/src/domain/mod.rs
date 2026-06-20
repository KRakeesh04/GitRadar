pub mod branch;
pub mod commit;
pub mod contributor;
pub mod repository;
pub mod value_objects;

// Re-export commonly used domain types
pub use branch::{Branch, BranchType};
pub use commit::{Commit, CommitInfo};
pub use contributor::Contributor;
pub use repository::Repository;
pub use value_objects::{ActivityLevel, HealthScore, RepositoryId};

// Result type for domain operations
pub type DomainResult<T> = Result<T, DomainError>;

// Domain-level errors
#[derive(Debug)]
pub enum DomainError {
    InvalidRepository(String),
    InvalidCommit(String),
    InvalidBranch(String),
    HealthCheckFailed(String),
    ActivityCalculationFailed(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::InvalidRepository(msg) => write!(f, "Invalid repository: {}", msg),
            DomainError::InvalidCommit(msg) => write!(f, "Invalid commit: {}", msg),
            DomainError::InvalidBranch(msg) => write!(f, "Invalid branch: {}", msg),
            DomainError::HealthCheckFailed(msg) => write!(f, "Health check failed: {}", msg),
            DomainError::ActivityCalculationFailed(msg) => {
                write!(f, "Activity calculation failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for DomainError {}
