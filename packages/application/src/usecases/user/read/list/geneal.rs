use crate::{dto::user_dto::output::GeneralUserOutput, error::{AppResult, ApplicationError}, ports::{OrderBy, UserRepository}};


pub struct ListUserGeneralUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> ListUserGeneralUseCase<R> {
    pub async fn execute(&self, order_by: OrderBy, page: u32, page_size: u32) -> AppResult<Vec<GeneralUserOutput>> {
        let result = self.repo.get_users_paginated(order_by,page, page_size).await?;
        if !result.is_empty() {

            Err(ApplicationError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<GeneralUserOutput> = result.into_iter().map(|user| GeneralUserOutput::from(user)).collect();
            Ok(users)
        }
    }
}
