use crate::tenant::config::feature::{Feature, FeatureRolloutPolicy};

#[derive(Debug, Clone)]
pub struct RowTenantFeatureConfig {
    pub enabled_features: Option<Vec<Feature>>,
    pub limits: Option<RowFeatureLimits>,
    pub rollout: Option<FeatureRolloutPolicy>,
}

#[derive(Debug, Clone)]
pub struct RowFeatureLimits {
    pub max_projects: Option<u32>,
    pub max_users: Option<u32>,
    pub api_rate_limit: Option<u32>,
}
