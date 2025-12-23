use domain::UserId;

use crate::{ dto::user_dto::output::OwnerUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};


pub struct GetUserByIdOwnerUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByIdOwnerUseCase<R> {
    pub async fn execute(&self, id: UserId) -> AppResult<OwnerUserOutput> {
        let result = self.repo.get_by_id(id.clone()).await?;
        match result {
            Some(user) => Ok(OwnerUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", id))),
        }
    }
}

