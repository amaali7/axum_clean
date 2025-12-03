use domain::Role;
use crate::{dto::role_dto::{input::CreateRoleInput, output::PrivilegeRoleOutput}, error::AppResult, ports::RoleRepository};


pub struct CreateRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> CreateRoleUseCase<R> {
    pub async fn execute(&self, input: CreateRoleInput) -> AppResult<PrivilegeRoleOutput> {
        let mut role_builder = Role::new(input.id);
        for permission in input.permissions.into_iter() {
             role_builder.add_permissions(permission);
        }
        for event in input.events.into_iter() {
             role_builder.add_events(event);
        }
        role_builder
            .set_created_at(input.created_at)
            .set_description(input.description)
            .set_is_system_role(input.is_system_role)
            .set_name(input.name);
        let role = role_builder.build()?;
        self.repo.save(&role).await?;
        Ok(PrivilegeRoleOutput::from(role))
    }
}
