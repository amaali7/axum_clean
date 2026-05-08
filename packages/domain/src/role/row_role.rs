use std::collections::HashSet;

use crate::{permissions::row_permissions::RowPermission, DateTime, Description, Name, RoleId};

#[derive(Debug, Clone)]
pub struct RowRole {
    pub id: Option<RoleId>,
    pub name: Option<Name>,
    pub description: Option<Description>,
    pub permissions: HashSet<RowPermission>,
    pub is_system_role: Option<bool>,
    pub created_at: Option<DateTime>,
    pub version: Option<u64>,
}
