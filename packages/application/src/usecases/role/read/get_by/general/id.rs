use domain::UserId;

use crate::{dto::user_dto::output::GeneralUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};

pub struct GetRoleByIdGenaralUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetRoleByIdGenaralUseCase<R> {
    pub async fn execute(&self, id: UserId) -> AppResult<GeneralUserOutput> {
        let result = self.repo.get_by_id(id.clone()).await?;
        match result {
            Some(user) => {
                Ok(GeneralUserOutput::from(user))
            },
            None => Err(ApplicationError::Repository(format!("User : {} not found", id))),
        }
    }
}
