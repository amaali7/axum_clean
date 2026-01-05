pub mod preferences;
pub mod profile;

pub use preferences::UserPreferencesField;
pub use profile::UserProfileField;

use super::{FieldPath, PathSegment, QueryField};
#[derive(Debug, Clone, Copy)]
pub enum UserField {
    Id,
    Email,
    Username,
    Profile,
    Roles,
    Permissions,
    Preferences,
    Status,
    Events,
}

impl QueryField for UserField {
    fn path(&self) -> FieldPath {
        vec![PathSegment::Field(
            match self {
                Self::Id => "id",
                Self::Email => "email",
                Self::Username => "username",
                Self::Profile => "profile",
                Self::Roles => "roles",
                Self::Permissions => "permissions",
                Self::Preferences => "preferences",
                Self::Status => "status",
                Self::Events => "events",
            }
            .into(),
        )]
    }

    fn table_ref(&self) -> &str {
        "user"
    }
}
