pub mod permissions;

pub use permissions::Permission;

use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoleId(uuid::Uuid);

impl RoleId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn system_role(id: &str) -> Self {
        Self(uuid::Uuid::new_v5(
            &uuid::Uuid::NAMESPACE_OID,
            id.as_bytes(),
        ))
    }
}

impl Deref for RoleId {
    type Target = uuid::Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RoleId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    id: RoleId,
    name: String,
    description: String,
    permissions: HashSet<Permission>,
    is_system_role: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl Role {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: RoleId::new(),
            name,
            description,
            permissions: HashSet::new(),
            is_system_role: false,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn system_role(name: String, description: String, permissions: Vec<Permission>) -> Self {
        Self {
            id: RoleId::system_role(&name),
            name,
            description,
            permissions: permissions.into_iter().collect(),
            is_system_role: true,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn add_permission(&mut self, permission: Permission) -> bool {
        self.permissions.insert(permission)
    }

    pub fn remove_permission(&mut self, permission: &Permission) -> bool {
        self.permissions.remove(permission)
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }

    pub fn can_grant_permission(&self, permission: &Permission) -> bool {
        // System roles can grant any permission, regular roles have limits
        self.is_system_role || !permission.is_sensitive()
    }

    // Getters
    pub fn id(&self) -> &RoleId {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn permissions(&self) -> &HashSet<Permission> {
        &self.permissions
    }
    pub fn is_system_role(&self) -> bool {
        self.is_system_role
    }
}

// Pre-defined system roles
impl Role {
    pub fn admin() -> Self {
        Self::system_role(
            "admin".to_string(),
            "System administrator with full access".to_string(),
            Permission::all(),
        )
    }

    pub fn manager() -> Self {
        Self::system_role(
            "manager".to_string(),
            "Manager who can review and approve reports".to_string(),
            vec![
                Permission::ViewReports,
                Permission::ReviewReports,
                Permission::ApproveReports,
                Permission::ManageUsers,
            ],
        )
    }

    pub fn user() -> Self {
        Self::system_role(
            "user".to_string(),
            "Regular user who can create and view their own reports".to_string(),
            vec![
                Permission::CreateReport,
                Permission::ViewOwnReports,
                Permission::EditOwnReports,
            ],
        )
    }

    pub fn auditor() -> Self {
        Self::system_role(
            "auditor".to_string(),
            "Auditor who can view all reports but not modify".to_string(),
            vec![Permission::ViewReports, Permission::ExportReports],
        )
    }
}
