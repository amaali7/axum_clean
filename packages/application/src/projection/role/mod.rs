use crate::authorization::access_descriptor::AccessableField;

pub mod projector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoleField {
    Id,
    Name,
    Description,
    Permissions(PermissionField),
    IsSystemRole,
    CreatedAt,
    Version,
}

impl AccessableField for RoleField {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionField {
    Id,
    Resource,
    Action,
    Description,
    CreatedAt,
    Version,
}

impl AccessableField for PermissionField {}
