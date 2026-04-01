use std::collections::HashSet;

use domain::{DateTime, RoleId, TenantId, UserId};

#[derive(Debug, Clone)]
pub struct MembershipView {
    pub user_id: Option<UserId>,
    pub tenet_id: Option<TenantId>,
    pub roles: Option<HashSet<RoleId>>,
    pub created_at: Option<DateTime>,
    pub version: Option<u64>,
}
