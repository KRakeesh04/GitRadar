use std::path::PathBuf;

use super::value_objects::{ActivityLevel, CommitCount, HealthScore, RepositoryId};
use super::DomainResult;

#[derive(Debug, Clone)]
pub struct Repository {
    // Identity
    pub id: RepositoryId,
    pub root_ids: Vec<i64>,
    pub root_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub path: PathBuf,
    pub git_dir: PathBuf,

    // Business State
    pub is_enabled: bool,
    pub is_starred: bool,
    pub starred_at: Option<String>,
    pub health_score: HealthScore,
    pub activity_level: ActivityLevel,

    // Metadata
    pub default_branch: Option<String>,
    pub head_branch: Option<String>,
    pub remote_url: Option<String>,
    pub is_dirty: bool,

    // Metrics (calculated from commits)
    pub total_commits: CommitCount,
    pub unique_contributors: u32,
}

impl Repository {
    pub fn new(
        id: i64,
        name: String,
        path: PathBuf,
        git_dir: PathBuf,
        remote_url: Option<String>,
        default_branch: Option<String>,
        head_branch: Option<String>,
    ) -> DomainResult<Self> {
        let repo_id = RepositoryId::new(id).map_err(super::DomainError::InvalidRepository)?;

        Ok(Repository {
            id: repo_id,
            root_ids: Vec::new(),
            root_id: None,
            created_at: String::new(),
            updated_at: String::new(),
            name,
            path,
            git_dir,
            is_enabled: true,
            is_starred: false,
            starred_at: None,
            health_score: HealthScore::new(0.5).unwrap(), // Default: Fair
            activity_level: ActivityLevel::VeryLow,
            default_branch,
            head_branch,
            remote_url,
            is_dirty: false,
            total_commits: CommitCount::new(0),
            unique_contributors: 0,
        })
    }

    // Is this repository healthy? (Health score >= 0.7)
    pub fn is_healthy(&self) -> bool {
        self.health_score.is_healthy()
    }

    pub fn status(&self) -> RepositoryStatus {
        match (self.is_healthy(), self.activity_level.is_active()) {
            (true, true) => RepositoryStatus::HealthyAndActive,
            (true, false) => RepositoryStatus::HealthyButInactive,
            (false, true) => RepositoryStatus::UnhealthyButActive,
            (false, false) => RepositoryStatus::UnhealthyAndInactive,
        }
    }

    // Determine if repository needs maintenance
    // Business rule: Poor health + low activity = needs maintenance
    pub fn needs_maintenance(&self) -> bool {
        !self.is_healthy() || !self.activity_level.is_active()
    }

    // Calculate repository risk score (0.0 to 1.0)
    // Considers: health, activity, commit frequency, contributor count
    pub fn calculate_risk_score(&self) -> f32 {
        let health_risk = 1.0 - self.health_score.value();
        let activity_risk = if self.activity_level.is_active() {
            0.0
        } else {
            0.3
        };
        let contributor_risk = if self.unique_contributors < 2 {
            0.2
        } else {
            0.0
        };
        let commit_risk = if self.total_commits.is_empty() {
            0.3
        } else {
            0.0
        };

        // Weighted calculation
        let total = (health_risk * 0.5)
            + (activity_risk * 0.25)
            + (contributor_risk * 0.15)
            + (commit_risk * 0.1);

        total.min(1.0)
    }

    // Get repository priority level for maintenance
    pub fn maintenance_priority(&self) -> MaintenancePriority {
        let risk = self.calculate_risk_score();
        match risk {
            r if r >= 0.8 => MaintenancePriority::Critical,
            r if r >= 0.6 => MaintenancePriority::High,
            r if r >= 0.4 => MaintenancePriority::Medium,
            r if r >= 0.2 => MaintenancePriority::Low,
            _ => MaintenancePriority::None,
        }
    }

    // Is this a dormant repository? (No commits, no activity)
    pub fn is_dormant(&self) -> bool {
        self.total_commits.is_empty() && !self.activity_level.is_active()
    }

    // Get activity trend description
    pub fn activity_description(&self) -> String {
        format!(
            "{} with {}",
            self.activity_level.description(),
            if self.unique_contributors == 0 {
                "no contributors".to_string()
            } else if self.unique_contributors == 1 {
                "1 contributor".to_string()
            } else {
                format!("{} contributors", self.unique_contributors)
            }
        )
    }

    // Get overall health report
    pub fn get_health_report(&self) -> HealthReport {
        HealthReport {
            overall_score: self.health_score.value(),
            status: self.health_score.status().to_string(),
            is_healthy: self.is_healthy(),
            activity_level: format!("{:?}", self.activity_level),
            repository_status: format!("{:?}", self.status()),
            risk_score: self.calculate_risk_score(),
            maintenance_needed: self.needs_maintenance(),
            is_dormant: self.is_dormant(),
        }
    }

    // Validate repository path exists and is accessible
    pub fn validate_path(&self) -> DomainResult<()> {
        if self.path.as_os_str().is_empty() {
            return Err(super::DomainError::InvalidRepository(
                "Repository path cannot be empty".to_string(),
            ));
        }

        if !self.git_dir.as_os_str().is_empty() && !self.git_dir.ends_with(".git") {
            return Err(super::DomainError::InvalidRepository(
                "Git directory must end with .git".to_string(),
            ));
        }

        Ok(())
    }

    pub fn set_health_score(&mut self, score: f32) -> DomainResult<()> {
        self.health_score = HealthScore::new(score).map_err(|e| {
            super::DomainError::InvalidRepository(format!("Invalid health score: {}", e))
        })?;
        Ok(())
    }

    pub fn set_activity_level(&mut self, level: ActivityLevel) {
        self.activity_level = level;
    }

    pub fn update_metrics(&mut self, total_commits: u32, unique_contributors: u32) {
        self.total_commits = CommitCount::new(total_commits);
        self.unique_contributors = unique_contributors;
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.is_dirty = dirty;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryStatus {
    HealthyAndActive,
    HealthyButInactive,
    UnhealthyButActive,
    UnhealthyAndInactive,
}

impl std::fmt::Display for RepositoryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryStatus::HealthyAndActive => write!(f, "Healthy & Active"),
            RepositoryStatus::HealthyButInactive => write!(f, "Healthy but Inactive"),
            RepositoryStatus::UnhealthyButActive => write!(f, "Unhealthy but Active"),
            RepositoryStatus::UnhealthyAndInactive => write!(f, "Unhealthy & Inactive"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaintenancePriority {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl std::fmt::Display for MaintenancePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaintenancePriority::None => write!(f, "No maintenance needed"),
            MaintenancePriority::Low => write!(f, "Low priority"),
            MaintenancePriority::Medium => write!(f, "Medium priority"),
            MaintenancePriority::High => write!(f, "High priority"),
            MaintenancePriority::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealthReport {
    pub overall_score: f32,
    pub status: String,
    pub is_healthy: bool,
    pub activity_level: String,
    pub repository_status: String,
    pub risk_score: f32,
    pub maintenance_needed: bool,
    pub is_dormant: bool,
}
#[derive(Debug, Clone)]
pub struct RepositoryCalculatedMetrics {
    pub total_commits: u32,
    pub weekly_commits: u32,
    pub unique_contributors: u32,
    pub health_score: HealthScore,
    pub activity_level: ActivityLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new(
            1,
            "test-repo".to_string(),
            PathBuf::from("/home/user/repos/test"),
            PathBuf::from("/home/user/repos/test/.git"),
            None,
            None,
            None,
        )
        .unwrap();

        assert_eq!(repo.id.value(), 1);
        assert_eq!(repo.name, "test-repo");
        assert!(!repo.is_healthy()); // Default score is 0.5
    }

    #[test]
    fn test_repository_health_status() {
        let mut repo = Repository::new(
            1,
            "test-repo".to_string(),
            PathBuf::from("/home/user/repos/test"),
            PathBuf::from("/home/user/repos/test/.git"),
            None,
            None,
            None,
        )
        .unwrap();

        repo.set_health_score(0.8).unwrap();
        assert!(repo.is_healthy());
    }

    #[test]
    fn test_repository_status() {
        let mut repo = Repository::new(
            1,
            "test-repo".to_string(),
            PathBuf::from("/home/user/repos/test"),
            PathBuf::from("/home/user/repos/test/.git"),
            None,
            None,
            None,
        )
        .unwrap();

        repo.set_health_score(0.8).unwrap();
        repo.set_activity_level(ActivityLevel::High);

        assert_eq!(repo.status(), RepositoryStatus::HealthyAndActive);
    }

    #[test]
    fn test_dormant_repository() {
        let repo = Repository::new(
            1,
            "test-repo".to_string(),
            PathBuf::from("/home/user/repos/test"),
            PathBuf::from("/home/user/repos/test/.git"),
            None,
            None,
            None,
        )
        .unwrap();

        assert!(repo.is_dormant()); // No commits, no activity
    }

    #[test]
    fn test_risk_score_calculation() {
        let mut repo = Repository::new(
            1,
            "test-repo".to_string(),
            PathBuf::from("/home/user/repos/test"),
            PathBuf::from("/home/user/repos/test/.git"),
            None,
            None,
            None,
        )
        .unwrap();

        repo.set_health_score(0.1).unwrap(); // Poor health
        repo.set_activity_level(ActivityLevel::VeryLow);
        repo.update_metrics(0, 0);

        let risk = repo.calculate_risk_score();
        assert!(risk > 0.5); // Should have significant risk
    }

    #[test]
    fn test_maintenance_priority() {
        let mut repo = Repository::new(
            1,
            "test-repo".to_string(),
            PathBuf::from("/home/user/repos/test"),
            PathBuf::from("/home/user/repos/test/.git"),
            None,
            None,
            None,
        )
        .unwrap();

        repo.set_health_score(0.0).unwrap();
        repo.set_activity_level(ActivityLevel::VeryLow);

        assert_eq!(repo.maintenance_priority(), MaintenancePriority::High);
    }

    #[test]
    fn test_health_report() {
        let mut repo = Repository::new(
            1,
            "test-repo".to_string(),
            PathBuf::from("/home/user/repos/test"),
            PathBuf::from("/home/user/repos/test/.git"),
            None,
            None,
            None,
        )
        .unwrap();

        repo.set_health_score(0.8).unwrap();
        repo.set_activity_level(ActivityLevel::High);
        repo.update_metrics(10, 2);
        let report = repo.get_health_report();

        assert!(report.is_healthy);
        assert!(!report.is_dormant);
    }
}
