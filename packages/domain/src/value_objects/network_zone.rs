#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkZone {
    TrustedCorporate,
    Internal,
    Partner,
    PublicInternet,
    Anonymous,
    HighRisk,
}
