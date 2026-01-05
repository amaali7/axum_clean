use crate::{dto::user_dto::output::PrivilegeUserOutput, error::{AppResult, ApplicationError}, ports::{OrderBy, UserRepository}};


pub struct ListUserPrivilegeUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> ListUserPrivilegeUseCase<R> {
    pub async fn execute(&self, order_by: OrderBy, page: u32, page_size: u32) -> AppResult<Vec<PrivilegeUserOutput>> {
        let result = self.repo.get_users_paginated(order_by,page, page_size).await?;
        if !result.is_empty() {
            Err(ApplicationError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<PrivilegeUserOutput> = result.into_iter().map(|user| PrivilegeUserOutput::from(user)).collect();
            Ok(users)
        }
    }
}

