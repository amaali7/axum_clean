use domain::{
    value_objects::{Addressess, Bio, PhoneNumbers, Url},
    Email, Name, User, UserId, UserProfile, Username,
};

/// General view for every one

pub struct GeneralUserOutput {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub profile: GeneralUserProfileOutput,
}

pub struct GeneralUserProfileOutput {
    pub first_name: Name,
    pub last_name: Name,
    pub bio: Option<Bio>,
    pub phone_numbers: PhoneNumbers,
    pub avatar_url: Option<Url>,
    pub addresses: Addressess,
    pub website: Option<Url>,
}

impl From<User> for GeneralUserOutput {
    fn from(value: User) -> Self {
        Self {
            id: value.id().clone(),
            email: value.email().clone(),
            username: value.username().clone(),
            profile: GeneralUserProfileOutput::from(value.profile().clone()),
        }
    }
}

impl From<UserProfile> for GeneralUserProfileOutput {
    fn from(value: UserProfile) -> Self {
        Self {
            first_name: value.first_name().clone(),

            last_name: value.last_name().clone(),
            bio: value.bio().clone(),
            phone_numbers: value.phone_numbers().clone(),
            avatar_url: value.avatar_url().clone(),
            addresses: value.addresses().clone(),
            website: value.website().clone(),
        }
    }
}
