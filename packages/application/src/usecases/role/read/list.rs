use crate::{RequestContex, dto::role_dto::output::PrivilegeRoleOutput, error::{AppResult, ApplicationError}, ports::{RoleRepository, SortBy}};


pub struct ListRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> ListRoleUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<PrivilegeRoleOutput>> {
        let result = self.repo.get_roles_paginated(ctx, sort_by,page, page_size).await?;
        if !result.is_empty() {
            Err(ApplicationError::Repository("Roles not found".to_string()))
        } else {
            let users: Vec<PrivilegeRoleOutput> = result.into_iter().map(|user| PrivilegeRoleOutput::from(user)).collect();
            Ok(users)
        }
    }
}
