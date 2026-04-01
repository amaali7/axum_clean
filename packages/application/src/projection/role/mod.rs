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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionField {
    Id,
    Resource,
    Action,
    Description,
    CreatedAt,
    Version,
}
