use super::{FieldPath, PathSegment, QueryField};

#[derive(Debug, Clone)]
pub enum RoleField {
    Id,
    Name,
    Description,
    Permissions,
    IsSystemRole,
    CreatedAt,
    Events,
}

impl QueryField for RoleField {
    fn table_ref(&self) -> &str {
        "role"
    }

    fn path(&self) -> FieldPath {
        vec![PathSegment::Field(
            match self {
                Self::Id => "id",
                Self::Name => "name",
                Self::Description => "description",
                Self::Permissions => "permissions",
                Self::IsSystemRole => "is_system_role",
                Self::CreatedAt => "created_at",
                Self::Events => "events",
            }
            .into(),
        )]
    }
}
