use crate::{dto::OwnerUserOutput, error::ApplicationError, ports::UserRepository};


pub struct ListUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> ListUserUseCase<R> {
    pub async fn execute(&self) -> Result<Vec<OwnerUserOutput>, ApplicationError> {
        let result = self.repo.list().await?;
        if !result.is_empty() {
            Err(ApplicationError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<OwnerUserOutput> = result.into_iter().map(|user| OwnerUserOutput::from(user)).collect();
            Ok(users)
        }
    }
}
