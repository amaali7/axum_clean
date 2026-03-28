use crate::value_objects::Conutry;

#[derive(Debug, Clone)]
pub enum LocationZone {
    InternalNetwork,
    CorporateOffice,
    Country(Conutry),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct EnvironmentLocation {
    zone: LocationZone,
}

impl EnvironmentLocation {
    pub fn new(zone: LocationZone) -> Self {
        Self { zone }
    }
    pub fn zone(&self) -> &LocationZone {
        &self.zone
    }
}
