use super::DomainResult;

#[derive(Debug, Clone)]
pub struct Branch {
    pub id: BranchId,
    pub repo_id: i64,
    pub name: String,
    pub branch_type: BranchType,

    // Metadata
    pub is_head: bool,
    pub is_default: bool,
    pub last_commit_hash: Option<String>,
    pub ahead_count_from_remote: u32,
    pub behind_count_from_remote: u32,
    pub ahead_count_from_default: u32,
    pub behind_count_from_default: u32,
}

impl Branch {
    // Create a new branch domain entity
    pub fn new(
        id: i64,
        repo_id: i64,
        name: String,
        is_head: bool,
        is_default: bool,
    ) -> DomainResult<Self> {
        if name.is_empty() {
            return Err(super::DomainError::InvalidBranch(
                "Branch name cannot be empty".to_string(),
            ));
        }

        let branch_type = BranchType::from_name(&name);

        Ok(Branch {
            id: BranchId(id),
            repo_id,
            name,
            branch_type,
            is_head,
            is_default,
            last_commit_hash: None,
            ahead_count_from_remote: 0,
            behind_count_from_remote: 0,
            ahead_count_from_default: 0,
            behind_count_from_default: 0,
        })
    }

    pub fn is_ahead(&self) -> bool {
        self.ahead_count_from_default > 0
    }

    pub fn is_behind(&self) -> bool {
        self.behind_count_from_default > 0
    }

    pub fn is_in_sync(&self) -> bool {
        self.ahead_count_from_default == 0 && self.behind_count_from_default == 0
    }

    // Get branch status
    pub fn status(&self) -> BranchStatus {
        match (self.is_ahead(), self.is_behind()) {
            (true, true) => BranchStatus::Diverged,
            (true, false) => BranchStatus::Ahead,
            (false, true) => BranchStatus::Behind,
            (false, false) => BranchStatus::InSync,
        }
    }

    // Get sync message for UI
    pub fn sync_message(&self) -> String {
        match self.status() {
            BranchStatus::InSync => "In sync with default".to_string(),
            BranchStatus::Ahead => format!(
                "Ahead by {} commit{} from default",
                self.ahead_count_from_default,
                if self.ahead_count_from_default == 1 {
                    ""
                } else {
                    "s"
                }
            ),
            BranchStatus::Behind => format!(
                "Behind by {} commit{} from default",
                self.behind_count_from_default,
                if self.behind_count_from_default == 1 {
                    ""
                } else {
                    "s"
                }
            ),
            BranchStatus::Diverged => {
                format!(
                    "Ahead by {} commit{} and behind by {} commit{} from default",
                    self.ahead_count_from_default,
                    if self.ahead_count_from_default == 1 {
                        ""
                    } else {
                        "s"
                    },
                    self.behind_count_from_default,
                    if self.behind_count_from_default == 1 {
                        ""
                    } else {
                        "s"
                    }
                )
            }
        }
    }

    // Recommend merging if ahead and not behind from default
    pub fn should_merge(&self) -> bool {
        self.is_ahead() && !self.is_behind() && !self.is_default
    }

    // Behind by many commits
    pub fn is_stale(&self) -> bool {
        self.behind_count_from_default > 50 && !self.is_head
    }

    // Determine branch importance
    pub fn importance(&self) -> BranchImportance {
        match self.branch_type {
            BranchType::Main => BranchImportance::Critical,
            BranchType::Develop => BranchImportance::High,
            BranchType::Release => BranchImportance::High,
            BranchType::Hotfix => BranchImportance::High,
            BranchType::Feature => BranchImportance::Medium,
            BranchType::Other => {
                if self.is_head {
                    BranchImportance::Medium
                } else {
                    BranchImportance::Low
                }
            }
        }
    }

    // Update sync information
    pub fn update_sync_info(&mut self, ahead: u32, behind: u32) {
        self.ahead_count_from_default = ahead;
        self.behind_count_from_default = behind;
    }

    // Set last commit hash
    pub fn set_last_commit(&mut self, hash: String) {
        self.last_commit_hash = Some(hash);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BranchId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchType {
    Main,    // main or master
    Develop, // develop
    Release, // release/* or release-*
    Hotfix,  // hotfix/* or hotfix-*
    Feature, // feature/* or feature-*
    Other,   // everything else
}

impl BranchType {
    // Determine branch type from name
    pub fn from_name(name: &str) -> Self {
        match name {
            "main" | "master" => BranchType::Main,
            "develop" | "development" => BranchType::Develop,
            name if name.starts_with("release/") || name.starts_with("release-") => {
                BranchType::Release
            }
            name if name.starts_with("hotfix/")
                || name.starts_with("hotfix-")
                || name.starts_with("fix/")
                || name.starts_with("fix-") =>
            {
                BranchType::Hotfix
            }
            name if name.starts_with("feature/") || name.starts_with("feature-") => {
                BranchType::Feature
            }
            _ => BranchType::Other,
        }
    }

    // Is this a special branch?
    pub fn is_special(&self) -> bool {
        matches!(
            self,
            BranchType::Main | BranchType::Develop | BranchType::Release | BranchType::Hotfix
        )
    }

    // Get branch template name
    pub fn template_name(&self) -> &'static str {
        match self {
            BranchType::Main => "main/master",
            BranchType::Develop => "develop",
            BranchType::Release => "release/*",
            BranchType::Hotfix => "fix/*",
            BranchType::Feature => "feature/*",
            BranchType::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchStatus {
    InSync,
    Ahead,
    Behind,
    Diverged,
}

impl std::fmt::Display for BranchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BranchStatus::InSync => write!(f, "In Sync"),
            BranchStatus::Ahead => write!(f, "Ahead"),
            BranchStatus::Behind => write!(f, "Behind"),
            BranchStatus::Diverged => write!(f, "Diverged"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BranchImportance {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_type_from_name() {
        assert_eq!(BranchType::from_name("main"), BranchType::Main);
        assert_eq!(BranchType::from_name("master"), BranchType::Main);
        assert_eq!(BranchType::from_name("develop"), BranchType::Develop);
        assert_eq!(
            BranchType::from_name("feature/new-feature"),
            BranchType::Feature
        );
        assert_eq!(BranchType::from_name("release/v1.0"), BranchType::Release);
        assert_eq!(
            BranchType::from_name("hotfix/critical-bug"),
            BranchType::Hotfix
        );
    }

    #[test]
    fn test_branch_status() {
        let mut branch = Branch::new(1, 1, "feature/test".to_string(), false, false).unwrap();

        branch.update_sync_info(0, 0);
        assert_eq!(branch.status(), BranchStatus::InSync);

        branch.update_sync_info(3, 0);
        assert_eq!(branch.status(), BranchStatus::Ahead);

        branch.update_sync_info(0, 3);
        assert_eq!(branch.status(), BranchStatus::Behind);

        branch.update_sync_info(3, 3);
        assert_eq!(branch.status(), BranchStatus::Diverged);
    }

    #[test]
    fn test_should_merge() {
        let mut branch = Branch::new(1, 1, "feature/test".to_string(), false, false).unwrap();

        branch.update_sync_info(3, 0);
        assert!(branch.should_merge()); // Ahead and not behind

        branch.update_sync_info(3, 1);
        assert!(!branch.should_merge()); // Behind - diverged

        let mut main = Branch::new(2, 1, "main".to_string(), true, true).unwrap();
        main.update_sync_info(3, 0);
        assert!(!main.should_merge()); // Main branch
    }

    #[test]
    fn test_branch_importance() {
        let main = Branch::new(1, 1, "main".to_string(), true, true).unwrap();
        assert_eq!(main.importance(), BranchImportance::Critical);

        let develop = Branch::new(2, 1, "develop".to_string(), false, false).unwrap();
        assert_eq!(develop.importance(), BranchImportance::High);

        let feature = Branch::new(3, 1, "feature/new".to_string(), false, false).unwrap();
        assert_eq!(feature.importance(), BranchImportance::Medium);
    }
}
