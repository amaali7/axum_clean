use crate::{
    tenant::config::collaboration::{
        CollaborationAccessLevel, CollaborationMode, FederationPolicy,
    },
    TenantId,
};

#[derive(Debug, Clone)]
pub struct RowTenantCollaborationConfig {
    pub trusted_tenants: Option<Vec<TenantId>>,
    pub allowed_modes: Option<Vec<CollaborationMode>>,
    pub federation: Option<FederationPolicy>,
    pub access_level: Option<CollaborationAccessLevel>,
}
