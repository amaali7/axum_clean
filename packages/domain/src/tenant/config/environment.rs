use crate::tenant::value_objects::{network_zone::NetworkZone, time_window::TimeWindow};

pub struct TenantEnvironmentConfig {
    allowed_time_window: Option<TimeWindow>,
    allowed_networks: Vec<NetworkZone>,
    require_managed_device: bool,
    max_risk_score: Option<u8>,
}

impl TenantEnvironmentConfig {
    pub fn new(
        allowed_time_window: Option<TimeWindow>,
        allowed_networks: &[NetworkZone],
        require_managed_device: bool,
        max_risk_score: Option<u8>,
    ) -> Self {
        Self {
            allowed_time_window,
            allowed_networks: allowed_networks.to_vec(),
            require_managed_device,
            max_risk_score,
        }
    }
    pub fn allowed_time_window(&self) -> &Option<TimeWindow> {
        &self.allowed_time_window
    }
    pub fn allowed_networks(&self) -> &Vec<NetworkZone> {
        &self.allowed_networks
    }
    pub fn require_managed_device(&self) -> bool {
        self.require_managed_device
    }
    pub fn max_risk_score(&self) -> &Option<u8> {
        &self.max_risk_score
    }
}
