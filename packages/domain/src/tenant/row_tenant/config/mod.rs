use authorization::RowTenantAuthorizationConfig;
use collaboration::RowTenantCollaborationConfig;
use environment::RowTenantEnvironmentConfig;
use feature::RowTenantFeatureConfig;

pub mod authorization;
pub mod collaboration;
pub mod environment;
pub mod feature;

#[derive(Debug, Clone)]
pub struct RowTenantConfig {
    pub authorization: Option<RowTenantAuthorizationConfig>,
    pub environment: Option<RowTenantEnvironmentConfig>,
    pub collaboration: Option<RowTenantCollaborationConfig>,
    pub features: Option<RowTenantFeatureConfig>,
}
