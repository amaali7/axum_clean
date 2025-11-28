
use domain::Email;

use crate::{dto::{GeneralUserOutput, OwnerUserOutput, PrivilegeUserOutput}, error::ApplicationError, ports::UserRepository};


pub struct GetUserUseByEmailOwnerCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserUseByEmailOwnerCase<R> {
    pub async fn execute(&self, email: Email ) -> Result<OwnerUserOutput, ApplicationError> {
        let result = self.repo.get_by_email(&email).await?;
        match result {
            Some(user) => Ok(OwnerUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", email))),
        }
    }
}


pub struct GetUserUseByEmailPrivilegeCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserUseByEmailPrivilegeCase<R> {
    pub async fn execute(&self, email: Email ) -> Result<PrivilegeUserOutput, ApplicationError> {
        let result = self.repo.get_by_email(&email).await?;
        match result {
            Some(user) => Ok(PrivilegeUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", email))),
        }
    }
}


pub struct GetUserUseByEmailGeneralCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserUseByEmailGeneralCase<R> {
    pub async fn execute(&self, email: Email ) -> Result<GeneralUserOutput, ApplicationError> {
        let result = self.repo.get_by_email(&email).await?;
        match result {
            Some(user) => Ok(GeneralUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", email))),
        }
    }
}
