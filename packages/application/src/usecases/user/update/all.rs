use domain::{value_objects::{Addressess, Bio, DateTime, PhoneNumbers, Url}, User, UserProfile};

use crate::{dto::{OwnerUserOutput, UpdateUserInput, UpdateUserProfileInput}, error::ApplicationError, ports::UserRepository};


pub struct UpdateUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UpdateUserUseCase<R> {
    pub async fn execute(&self, input: UpdateUserInput, state: User) -> Result<OwnerUserOutput, ApplicationError> {
        let user_profile_builder = UserProfile::new();
        let profile = input.profile.unwrap_or(UpdateUserProfileInput::from(state.profile().to_owned()));
        let user_profile = user_profile_builder
            .set_addresss(profile.addresses.unwrap_or(state.profile().addresses().clone().unwrap_or(Addressess::default())))
            .set_last_name(profile.last_name.unwrap_or(state.profile().last_name().clone()))
            .set_first_name(profile.first_name.unwrap_or(state.profile().first_name().clone()))
            .set_avatar_url(profile.avatar_url.unwrap_or(state.profile().avatar_url().clone().unwrap_or(Url::default())))
            .set_bio(profile.bio.unwrap_or(state.profile().bio().clone().unwrap_or(Bio::default())))
            .set_date_of_birth(profile.date_of_birth.unwrap_or(state.profile().date_of_birth().clone().unwrap_or(DateTime::new(1)?)))
            .set_phone_numbers(profile.phone_numbers.unwrap_or(state.profile().phone_numbers().clone().unwrap_or(PhoneNumbers::default())))
            .set_password(profile.password.unwrap_or(state.profile().password().clone()))
            .build(None,profile.updated_at)?;

        let user_builder = User::new(input.id)?;
        let user = user_builder
            .add_permissions(input.permissions.unwrap_or(state.permissions().clone()))
            .add_roles(input.roles.unwrap_or(state.roles().clone()))
            .set_email(input.email.unwrap_or(state.email().clone()))
            .set_profile(user_profile)
            .set_username(input.username.unwrap_or(state.username().clone()))
            .build()?;

        self.repo.update(&user).await?;
        Ok(OwnerUserOutput::from(user))
    }
}
