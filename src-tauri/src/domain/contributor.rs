/// Contributor Domain Entity
/// Pure business logic for contributors
use super::DomainResult;

/// Contributor - Core business entity for a repository contributor
#[derive(Debug, Clone)]
pub struct Contributor {
    pub id: ContributorId,
    pub repo_id: i64,
    pub name: String,
    pub email: String,

    // Metrics
    pub commit_count: u32,
    pub additions: u32,
    pub deletions: u32,
    pub active_days: u32,
    pub last_commit_at: Option<String>,
}

impl Contributor {
    pub fn new(id: i64, repo_id: i64, name: String, email: String) -> DomainResult<Self> {
        if name.is_empty() {
            return Err(super::DomainError::InvalidCommit(
                "Contributor name cannot be empty".to_string(),
            ));
        }

        Ok(Contributor {
            id: ContributorId(id),
            repo_id,
            name,
            email,
            commit_count: 0,
            additions: 0,
            deletions: 0,
            active_days: 0,
            last_commit_at: None,
        })
    }

    // Calculate contributor's impact score (0.0 to 1.0)
    // Based on: commits, changes, active days
    pub fn impact_score(&self) -> f32 {
        if self.commit_count == 0 {
            return 0.0;
        }

        let commit_weight = (self.commit_count as f32).log2() / 10.0;
        let change_weight = ((self.additions + self.deletions) as f32).log2() / 12.0;
        let consistency_weight = self.active_days as f32 / 365.0;

        let score = (commit_weight * 0.5) + (change_weight * 0.3) + (consistency_weight * 0.2);
        score.min(1.0)
    }

    // Get contributor level/tier
    pub fn contributor_level(&self) -> ContributorLevel {
        let impact = self.impact_score();
        match impact {
            s if s >= 0.8 => ContributorLevel::CoreMaintainer,
            s if s >= 0.6 => ContributorLevel::MajorContributor,
            s if s >= 0.4 => ContributorLevel::RegularContributor,
            s if s >= 0.2 => ContributorLevel::Occasional,
            _ => ContributorLevel::Minimal,
        }
    }

    // Average commits per active day
    pub fn commits_per_day(&self) -> f32 {
        if self.active_days == 0 {
            return 0.0;
        }
        self.commit_count as f32 / self.active_days as f32
    }

    // Average changes per commit
    pub fn changes_per_commit(&self) -> u32 {
        if self.commit_count == 0 {
            return 0;
        }
        (self.additions + self.deletions) / self.commit_count
    }

    // Active if they committed within last 30 days
    pub fn is_active(&self) -> bool {
        if let Some(last_commit) = &self.last_commit_at {
            // Simple check: if string contains recent date pattern
            // In production, compare with actual timestamps
            !last_commit.is_empty()
        } else {
            false
        }
    }

    pub fn get_role(&self) -> ContributorRole {
        match (self.contributor_level(), self.is_active()) {
            (ContributorLevel::CoreMaintainer, true) => ContributorRole::ActiveMaintainer,
            (ContributorLevel::CoreMaintainer, false) => ContributorRole::InactiveMaintainer,
            (ContributorLevel::MajorContributor, true) => ContributorRole::ActiveContributor,
            (ContributorLevel::MajorContributor, false) => ContributorRole::InactiveContributor,
            _ => ContributorRole::Contributor,
        }
    }

    pub fn contribution_summary(&self) -> String {
        format!(
            "{} commits, {} added, {} removed, {} active days",
            self.commit_count, self.additions, self.deletions, self.active_days
        )
    }

    // Update contributor metrics
    pub fn update_metrics(&mut self, additions: u32, deletions: u32, active_days: u32) {
        self.additions = additions;
        self.deletions = deletions;
        self.active_days = active_days;
    }

    pub fn increment_commit_count(&mut self) {
        self.commit_count += 1;
    }

    pub fn set_last_commit(&mut self, timestamp: String) {
        self.last_commit_at = Some(timestamp);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContributorId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContributorLevel {
    Minimal = 0,
    Occasional = 1,
    RegularContributor = 2,
    MajorContributor = 3,
    CoreMaintainer = 4,
}

impl std::fmt::Display for ContributorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContributorLevel::Minimal => write!(f, "Minimal"),
            ContributorLevel::Occasional => write!(f, "Occasional"),
            ContributorLevel::RegularContributor => write!(f, "Regular"),
            ContributorLevel::MajorContributor => write!(f, "Major"),
            ContributorLevel::CoreMaintainer => write!(f, "Core Maintainer"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContributorRole {
    ActiveMaintainer,
    InactiveMaintainer,
    ActiveContributor,
    InactiveContributor,
    Contributor,
}

impl std::fmt::Display for ContributorRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContributorRole::ActiveMaintainer => write!(f, "Active Maintainer"),
            ContributorRole::InactiveMaintainer => write!(f, "Inactive Maintainer"),
            ContributorRole::ActiveContributor => write!(f, "Active Contributor"),
            ContributorRole::InactiveContributor => write!(f, "Inactive Contributor"),
            ContributorRole::Contributor => write!(f, "Contributor"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contributor_creation() {
        let contributor =
            Contributor::new(1, 1, "John Doe".to_string(), "john@example.com".to_string()).unwrap();

        assert_eq!(contributor.name, "John Doe");
        assert_eq!(contributor.commit_count, 0);
    }

    #[test]
    fn test_contributor_level() {
        let mut contributor =
            Contributor::new(1, 1, "John Doe".to_string(), "john@example.com".to_string()).unwrap();

        contributor.commit_count = 100;
        contributor.additions = 10000;
        contributor.deletions = 5000;
        contributor.active_days = 200;

        assert_eq!(
            contributor.contributor_level(),
            ContributorLevel::CoreMaintainer
        );
    }

    #[test]
    fn test_impact_score() {
        let mut contributor =
            Contributor::new(1, 1, "John Doe".to_string(), "john@example.com".to_string()).unwrap();

        assert_eq!(contributor.impact_score(), 0.0); // No contributions

        contributor.commit_count = 50;
        contributor.additions = 5000;
        contributor.deletions = 2000;
        contributor.active_days = 100;

        let impact = contributor.impact_score();
        assert!(impact > 0.0 && impact <= 1.0);
    }

    #[test]
    fn test_commits_per_day() {
        let mut contributor =
            Contributor::new(1, 1, "John Doe".to_string(), "john@example.com".to_string()).unwrap();

        contributor.commit_count = 50;
        contributor.active_days = 10;

        assert_eq!(contributor.commits_per_day(), 5.0);
    }

    #[test]
    fn test_changes_per_commit() {
        let mut contributor =
            Contributor::new(1, 1, "John Doe".to_string(), "john@example.com".to_string()).unwrap();

        contributor.commit_count = 10;
        contributor.additions = 1000;
        contributor.deletions = 500;

        assert_eq!(contributor.changes_per_commit(), 150);
    }

    #[test]
    fn test_contribution_summary() {
        let mut contributor =
            Contributor::new(1, 1, "John Doe".to_string(), "john@example.com".to_string()).unwrap();

        contributor.commit_count = 50;
        contributor.additions = 5000;
        contributor.deletions = 2000;
        contributor.active_days = 100;

        let summary = contributor.contribution_summary();
        assert!(summary.contains("50 commits"));
        assert!(summary.contains("5000 added"));
    }

    #[test]
    fn test_contributor_role() {
        let mut contributor =
            Contributor::new(1, 1, "John Doe".to_string(), "john@example.com".to_string()).unwrap();

        contributor.commit_count = 100;
        contributor.additions = 10000;
        contributor.deletions = 5000;
        contributor.active_days = 200;
        contributor.set_last_commit("2024-01-01T10:00:00Z".to_string());

        assert_eq!(contributor.get_role(), ContributorRole::ActiveMaintainer);
    }
}
