use crate::tenant::environment::location::LocationZone;

#[derive(Debug, Clone)]
pub struct RowEnvironmentLocation {
    pub zone: Option<LocationZone>,
}
