use std::collections::HashSet;

use domain::{Permission, Role};

use crate::{
    authorization::access_descriptor::AccessControl,
    dto::role::view::{PermissionView, RoleView},
};

use super::{PermissionField, RoleField};

pub struct RoleProjector;

impl RoleProjector {
    pub fn project(
        role: Role,
        permissions: &[Permission],
        access: &AccessControl<RoleField>,
    ) -> RoleView {
        let allow = |f| access.readable_fields.contains(&f);
        let parts = role.into_parts();

        RoleView {
            version: allow(RoleField::Version).then(|| parts.version),
            id: allow(RoleField::Id).then(|| parts.id),
            name: allow(RoleField::Name).then(|| parts.name),
            description: allow(RoleField::Description).then(|| parts.description),
            permissions: if allow(RoleField::Permissions(PermissionField::Id)) {
                permissions
                    .into_iter()
                    .map(|permission| PermissionProjector::project(permission.clone(), access))
                    .collect::<HashSet<PermissionView>>()
            },
            is_system_role: allow(RoleField::IsSystemRole).then(|| parts.is_system_role),
            created_at: allow(RoleField::CreatedAt).then(|| parts.created_at),
        }
    }
}

pub struct PermissionProjector;

impl PermissionProjector {
    pub fn project(permission: Permission, access: &AccessControl<RoleField>) -> PermissionView {
        let parts = permission.into_parts();
        let allow = |f| access.readable_fields.contains(&RoleField::Permissions(f));

        PermissionView {
            id: allow(PermissionField::Id).then(|| parts.id),
            resource: allow(PermissionField::Resource).then(|| parts.resource),
            action: allow(PermissionField::Action).then(|| parts.action),
            description: allow(PermissionField::Description).then(|| parts.description),
            created_at: allow(PermissionField::CreatedAt).then(|| parts.created_at),
            version: allow(PermissionField::Version).then(|| parts.version),
        }
    }
}
