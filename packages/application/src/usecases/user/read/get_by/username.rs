use domain::Username;

use crate::{dto::{GeneralUserOutput, OwnerUserOutput, PrivilegeUserOutput}, error::ApplicationError, ports::UserRepository};


pub struct GetUserUseByUsernameOwnerCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserUseByUsernameOwnerCase<R> {
    pub async fn execute(&self, username: Username) -> Result<OwnerUserOutput, ApplicationError> {
        let result = self.repo.get_by_username(&username).await?;
        match result {
            Some(user) => Ok(OwnerUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", username))),
        }
    }
}


pub struct GetUserUseByUsernamePrivilegeCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserUseByUsernamePrivilegeCase<R> {
    pub async fn execute(&self, username: Username) -> Result<PrivilegeUserOutput, ApplicationError> {
        let result = self.repo.get_by_username(&username).await?;
        match result {
            Some(user) => Ok(PrivilegeUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", username))),
        }
    }
}


pub struct GetUserUseByUsernameGeneralCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserUseByUsernameGeneralCase<R> {
    pub async fn execute(&self, username: Username) -> Result<GeneralUserOutput, ApplicationError> {
        let result = self.repo.get_by_username(&username).await?;
        match result {
            Some(user) => Ok(GeneralUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", username))),
        }
    }
}
