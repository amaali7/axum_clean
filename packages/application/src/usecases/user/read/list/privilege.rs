use std::sync::Arc;

use crate::{SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::output::PrivilegeUserOutput, error::{AppError, AppResult}, ports::{SortBy, UserRepository}};


pub struct ListUserPrivilegeUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl ListUserPrivilegeUseCase {
    pub async fn execute(&self, ctx: SubjectContex, sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<PrivilegeUserOutput>> {
        let result = self.repo.get_users_paginated(ctx, sort_by,page, page_size).await?;
        if !result.is_empty() {
            Err(AppError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<PrivilegeUserOutput> = result.into_iter().map(|user| PrivilegeUserOutput::from(user)).collect();
            Ok(users)
        }
    }
}

