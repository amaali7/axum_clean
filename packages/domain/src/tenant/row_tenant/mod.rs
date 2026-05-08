pub mod config;
pub mod environment;
use crate::TenantId;
use config::RowTenantConfig;

use crate::{DateTime, Description, Name};

#[derive(Debug, Clone)]
pub struct RowTenant {
    pub id: Option<TenantId>,
    pub name: Option<Name>,
    pub description: Option<Description>,
    pub created_at: Option<DateTime>,
    pub config: Option<RowTenantConfig>,
    pub version: Option<u64>,
}
