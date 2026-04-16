use std::sync::Arc;

use domain::Email;

use crate::{ SubjectContex, authorization::ports::AuthorizationService,  error::AppResult, ports::UserRepository};

pub struct GetUserByEmailUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl GetUserByEmailUseCase {
    pub async fn execute(&self, ctx: SubjectContex, email: Email ) -> AppResult<UserView> {
        self.repo.get_by_email(ctx, email.clone()).await
    }
}
