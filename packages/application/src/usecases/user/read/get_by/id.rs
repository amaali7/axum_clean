
use domain::UserId;

use crate::{dto::{GeneralUserOutput, OwnerUserOutput, PrivilegeUserOutput}, error::ApplicationError, ports::UserRepository};


pub struct GetUserUseByIdOwnerCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserUseByIdOwnerCase<R> {
    pub async fn execute(&self, id: UserId) -> Result<OwnerUserOutput, ApplicationError> {
        let result = self.repo.get_by_id(&id).await?;
        match result {
            Some(user) => Ok(OwnerUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", id))),
        }
    }
}

pub struct GetUserUseByIdPrivilegeCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserUseByIdPrivilegeCase<R> {
    pub async fn execute(&self, id: UserId) -> Result<PrivilegeUserOutput, ApplicationError> {
        let result = self.repo.get_by_id(&id).await?;
        match result {
            Some(user) => Ok(PrivilegeUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", id))),
        }
    }
}

pub struct GetUserUseByIdGenaralCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserUseByIdGenaralCase<R> {
    pub async fn execute(&self, id: UserId) -> Result<GeneralUserOutput, ApplicationError> {
        let result = self.repo.get_by_id(&id).await?;
        match result {
            Some(user) => Ok(GeneralUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", id))),
        }
    }
}
