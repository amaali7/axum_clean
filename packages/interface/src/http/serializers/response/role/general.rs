use application::dto::role_dto::output::GeneralRoleOutput;

use crate::{
    common_objects::role::InterfaceRoleId,
    error::{InterfaceError, InterfaceResult},
    value_objects::{InterfaceDescription, InterfaceName},
};

pub struct GeneralRoleResponse {
    pub id: InterfaceRoleId,
    pub name: InterfaceName,
    pub description: InterfaceDescription,
}

impl TryFrom<GeneralRoleOutput> for GeneralRoleResponse {
    type Error = InterfaceError;

    fn try_from(value: GeneralRoleOutput) -> InterfaceResult<Self> {
        Ok(Self {
            id: value.id.into(),
            name: value.name.try_into()?,
            description: value.description.try_into()?,
        })
    }
}
