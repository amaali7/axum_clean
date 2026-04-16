use std::sync::Arc;

use domain::User;

use crate::{ SubjectContex, dto::{user::{command::UserCommand, view::UserView}},
                                   error::AppResult, ports::UserRepository, usecases::usecase_discriptor::UseCaseDescriptor;
use crate::authorization::ports::AuthorizationService;


pub struct UpdateUserUseCase{
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl UpdateUserUseCase {
    pub async fn execute(&self, ctx: SubjectContex, input: UserCommand) -> AppResult<UserView> {
        self.repo.update( ctx, input).await
    }
}

impl UseCaseDescriptor for UpdateUserUseCase {
    const NAME: &'static str = "update_user";

    const RESOURCE: &'static str= "user";

    const ACTION: &'static str = "update";
}
