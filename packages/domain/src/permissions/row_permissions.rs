use crate::{
    value_objects::{Action, Resource},
    DateTime, Description,
};

use super::PermissionId;

#[derive(Debug, Clone, Hash)]
pub struct RowPermission {
    pub id: Option<PermissionId>,
    pub resource: Option<Resource>,
    pub action: Option<Action>,
    pub description: Option<Description>,
    pub created_at: Option<DateTime>,
    pub version: Option<u64>,
}
