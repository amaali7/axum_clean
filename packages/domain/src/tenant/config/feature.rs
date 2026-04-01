#[derive(Debug, Clone)]
pub struct TenantFeatureConfig {
    enabled_features: Vec<Feature>,
    limits: FeatureLimits,
    rollout: FeatureRolloutPolicy,
}

#[derive(Debug, Clone)]
pub struct TenantFeatureConfigParts {
    pub enabled_features: Vec<Feature>,
    pub limits: FeatureLimits,
    pub rollout: FeatureRolloutPolicy,
}

impl TenantFeatureConfig {
    pub fn new(
        enabled_features: Vec<Feature>,
        limits: FeatureLimits,
        rollout: FeatureRolloutPolicy,
    ) -> Self {
        Self {
            enabled_features,
            limits,
            rollout,
        }
    }

    pub fn into_parts(self) -> TenantFeatureConfigParts {
        let Self {
            enabled_features,
            limits,
            rollout,
        } = self;
        TenantFeatureConfigParts {
            enabled_features,
            limits,
            rollout,
        }
    }

    // Getters

    pub fn enabled_features(&self) -> &Vec<Feature> {
        &self.enabled_features
    }
    pub fn limits(&self) -> &FeatureLimits {
        &self.limits
    }
    pub fn rollout(&self) -> &FeatureRolloutPolicy {
        &self.rollout
    }
    pub fn is_enabled(&self, feature: Feature) -> bool {
        self.enabled_features.contains(&feature)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    AdvancedReports,
    CrossTenantSharing,
    AuditLogs,
    ApiAccess,
    RiskScoring,
    BetaDashboard,
}
#[derive(Debug, Clone)]
pub struct FeatureLimits {
    max_projects: u32,
    max_users: u32,
    api_rate_limit: u32,
}

#[derive(Debug, Clone)]
pub struct FeatureLimitsParts {
    pub max_projects: u32,
    pub max_users: u32,
    pub api_rate_limit: u32,
}

impl FeatureLimits {
    pub fn new(max_projects: u32, max_users: u32, api_rate_limit: u32) -> Self {
        Self {
            max_projects,
            max_users,
            api_rate_limit,
        }
    }

    pub fn into_parts(self) -> FeatureLimitsParts {
        let FeatureLimits {
            max_projects,
            max_users,
            api_rate_limit,
        } = self;
        FeatureLimitsParts {
            max_projects,
            max_users,
            api_rate_limit,
        }
    }

    pub fn max_projects(&self) -> &u32 {
        &self.max_projects
    }
    pub fn max_users(&self) -> &u32 {
        &self.max_users
    }
    pub fn api_rate_limit(&self) -> &u32 {
        &self.api_rate_limit
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FeatureRolloutPolicy {
    Stable,
    Beta,
    Canary,
}
