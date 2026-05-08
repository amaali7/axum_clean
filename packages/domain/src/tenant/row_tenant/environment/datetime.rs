use crate::DateTime;

#[derive(Debug, Clone)]
pub struct RowEnvironmentTime {
    pub timestamp: Option<DateTime>,
    pub is_business_hours: Option<bool>,
}
