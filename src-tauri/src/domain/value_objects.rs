// Health Score: 0.0 to 1.0 indicating repository health
// Pure business value - calculated from repository metrics
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HealthScore(f32);

impl HealthScore {
    // Create a new health score with validation (0.0 to 1.0)
    pub fn new(score: f32) -> Result<Self, String> {
        if score < 0.0 || score > 1.0 {
            return Err("Health score must be between 0.0 and 1.0".to_string());
        }
        Ok(HealthScore(score))
    }

    // Get the numeric value
    pub fn value(&self) -> f32 {
        self.0
    }

    // Determine health status from score
    pub fn status(&self) -> HealthStatus {
        match self.0 {
            score if score >= 0.8 => HealthStatus::Excellent,
            score if score >= 0.6 => HealthStatus::Good,
            score if score >= 0.4 => HealthStatus::Fair,
            score if score >= 0.2 => HealthStatus::Poor,
            _ => HealthStatus::Critical,
        }
    }

    // Check if repository is healthy (>= 0.7)
    pub fn is_healthy(&self) -> bool {
        self.0 >= 0.7
    }

    // Get health description for UI
    pub fn description(&self) -> &'static str {
        match self.status() {
            HealthStatus::Excellent => "Repository is in excellent condition",
            HealthStatus::Good => "Repository is in good condition",
            HealthStatus::Fair => "Repository has some issues",
            HealthStatus::Poor => "Repository needs attention",
            HealthStatus::Critical => "Repository requires immediate attention",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Excellent => write!(f, "Excellent"),
            HealthStatus::Good => write!(f, "Good"),
            HealthStatus::Fair => write!(f, "Fair"),
            HealthStatus::Poor => write!(f, "Poor"),
            HealthStatus::Critical => write!(f, "Critical"),
        }
    }
}

// Activity Level: Describes repository commit frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActivityLevel {
    VeryLow,  // < 1 commit per week
    Low,      // 1-3 commits per week
    Moderate, // 4-10 commits per week
    High,     // 11-20 commits per week
    VeryHigh, // > 20 commits per week
}

impl ActivityLevel {
    // Calculate activity level from commits in last 7 days
    // Pure business logic - no database access
    pub fn from_weekly_commits(commit_count: u32) -> Self {
        match commit_count {
            0..=0 => ActivityLevel::VeryLow,
            1..=3 => ActivityLevel::Low,
            4..=10 => ActivityLevel::Moderate,
            11..=20 => ActivityLevel::High,
            _ => ActivityLevel::VeryHigh,
        }
    }

    // Human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            ActivityLevel::VeryLow => "Very Low - Rarely updated",
            ActivityLevel::Low => "Low - Occasional updates",
            ActivityLevel::Moderate => "Moderate - Regular updates",
            ActivityLevel::High => "High - Frequent updates",
            ActivityLevel::VeryHigh => "Very High - Very active development",
        }
    }

    // Is this repository actively maintained?
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            ActivityLevel::Moderate | ActivityLevel::High | ActivityLevel::VeryHigh
        )
    }
}

// Repository ID - Value object for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RepositoryId(pub i64);

impl RepositoryId {
    pub fn new(id: i64) -> Result<Self, String> {
        if id <= 0 {
            return Err("Repository ID must be positive".to_string());
        }
        Ok(RepositoryId(id))
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

// Commit Count within a period
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommitCount(pub u32);

impl CommitCount {
    pub fn new(count: u32) -> Self {
        CommitCount(count)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl From<u32> for CommitCount {
    fn from(value: u32) -> Self {
        CommitCount(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_score_creation() {
        assert!(HealthScore::new(0.5).is_ok());
        assert!(HealthScore::new(1.0).is_ok());
        assert!(HealthScore::new(0.0).is_ok());
        assert!(HealthScore::new(1.5).is_err());
        assert!(HealthScore::new(-0.1).is_err());
    }

    #[test]
    fn test_health_status() {
        assert_eq!(
            HealthScore::new(0.9).unwrap().status(),
            HealthStatus::Excellent
        );
        assert_eq!(HealthScore::new(0.7).unwrap().status(), HealthStatus::Good);
        assert_eq!(HealthScore::new(0.5).unwrap().status(), HealthStatus::Fair);
    }

    #[test]
    fn test_activity_level_calculation() {
        assert_eq!(
            ActivityLevel::from_weekly_commits(0),
            ActivityLevel::VeryLow
        );
        assert_eq!(ActivityLevel::from_weekly_commits(2), ActivityLevel::Low);
        assert_eq!(
            ActivityLevel::from_weekly_commits(7),
            ActivityLevel::Moderate
        );
        assert_eq!(ActivityLevel::from_weekly_commits(15), ActivityLevel::High);
        assert_eq!(
            ActivityLevel::from_weekly_commits(25),
            ActivityLevel::VeryHigh
        );
    }

    #[test]
    fn test_activity_is_active() {
        assert!(!ActivityLevel::VeryLow.is_active());
        assert!(!ActivityLevel::Low.is_active());
        assert!(ActivityLevel::Moderate.is_active());
        assert!(ActivityLevel::High.is_active());
    }

    #[test]
    fn test_repository_id_validation() {
        assert!(RepositoryId::new(1).is_ok());
        assert!(RepositoryId::new(0).is_err());
        assert!(RepositoryId::new(-1).is_err());
    }
}
