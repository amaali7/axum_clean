#[derive(Debug, Clone)]
pub struct RowDeviceSecurityPosture {
    pub is_managed: Option<bool>,
    pub is_compliant: Option<bool>,
    pub has_recent_security_patch: Option<bool>,
}
