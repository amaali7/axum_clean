use std::sync::Arc;

use domain::Username;

use crate::{SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::output::PrivilegeUserOutput, error::AppResult, ports::UserRepository};

pub struct GetUserByUsernamePrivilegeUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl GetUserByUsernamePrivilegeUseCase {
    pub async fn execute(&self, ctx: SubjectContex, username: Username) -> AppResult<PrivilegeUserOutput> {
        Ok(self.repo.get_by_username(ctx, username.clone()).await?.into())
    }
}
