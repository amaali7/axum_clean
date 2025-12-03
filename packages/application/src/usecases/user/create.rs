use domain::{User, UserProfile};

use crate::{ dto::user_dto::{input::CreateUserInput, output::OwnerUserOutput}, error::AppResult, ports::UserRepository};


pub struct CreateUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub async fn execute(&self, input: CreateUserInput) -> AppResult<OwnerUserOutput> {
        let mut user_profile_builder = UserProfile::new();
        user_profile_builder
            .set_addresss(input.profile.addresses)
            .set_last_name(input.profile.last_name)
            .set_first_name(input.profile.first_name)
            .set_avatar_url(input.profile.avatar_url)
            .set_bio(input.profile.bio)
            .set_date_of_birth(input.profile.date_of_birth)
            .set_phone_numbers(input.profile.phone_numbers)
            .set_password(input.profile.password);
          let user_profile =   user_profile_builder.build(None, input.profile.updated_at)?;

        let mut user_builder = User::new(input.id)?;
        user_builder
            .add_permissions(input.permissions)
            .add_roles(input.roles)
            .set_email(input.email)
            .set_profile(user_profile)
            .set_username(input.username);
            let user = user_builder.build()?;

        self.repo.save(&user).await?;
        Ok(OwnerUserOutput::from(user))
    }
}
