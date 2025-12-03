use domain::{value_objects::Description, Name, Role, RoleId};

pub struct GeneralRoleOutput {
    pub id: RoleId,
    pub name: Name,
    pub description: Description,
}

impl From<Role> for GeneralRoleOutput {
    fn from(value: Role) -> Self {
        Self {
            id: value.id(),
            name: value.name(),
            description: value.description(),
        }
    }
}
