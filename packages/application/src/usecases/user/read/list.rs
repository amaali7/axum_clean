use std::sync::Arc;

use crate::{SubjectContex, authorization::ports::AuthorizationService, dto::user::view::UserView, error::{AppError, AppResult}, ports::{SortBy, UserRepository}};


pub struct ListUserUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl ListUserUseCase {
    pub async fn execute(&self, ctx: SubjectContex, sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<UserView>> {
        let result = self.repo.get_users_paginated(ctx, sort_by,page, page_size).await?;
        if !result.is_empty() {
            Err(AppError::Repository("Users not found".to_string()))
        } else {
            Ok(result)
        }
    }
}

