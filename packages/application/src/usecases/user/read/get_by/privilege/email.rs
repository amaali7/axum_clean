use std::sync::Arc;

use domain::Email;

use crate::{ SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::output::PrivilegeUserOutput, error::AppResult, ports::UserRepository};

pub struct GetUserByEmailPrivilegeUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl GetUserByEmailPrivilegeUseCase {
    pub async fn execute(&self, ctx: SubjectContex, email: Email ) -> AppResult<PrivilegeUserOutput> {
        Ok(self.repo.get_by_email(ctx, email.clone()).await?.into())
    }
}
