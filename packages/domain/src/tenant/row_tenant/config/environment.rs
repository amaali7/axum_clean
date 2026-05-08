use crate::value_objects::{NetworkZone, TimeWindow};

#[derive(Debug, Clone)]
pub struct RowTenantEnvironmentConfig {
    pub allowed_time_window: Option<TimeWindow>,
    pub allowed_networks: Option<Vec<NetworkZone>>,
    pub require_managed_device: Option<bool>,
    pub max_risk_score: Option<u8>,
}
