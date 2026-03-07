use std::sync::Arc;

use crate::{SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::output::GeneralUserOutput, error::{AppError, AppResult}, ports::{SortBy, UserRepository}};


pub struct ListUserGeneralUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl ListUserGeneralUseCase {
    pub async fn execute(&self, ctx: SubjectContex, sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<GeneralUserOutput>> {
        let result = self.repo.get_users_paginated(ctx, sort_by,page, page_size).await?;
        if !result.is_empty() {

            Err(AppError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<GeneralUserOutput> = result.into_iter().map(|user| GeneralUserOutput::from(user)).collect();
            Ok(users)
        }
    }
}
