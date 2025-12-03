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
    pub phone_numbers: Option<PhoneNumbers>,
    pub avatar_url: Option<Url>,
    pub addresses: Option<Addressess>,
    pub website: Option<Url>,
}

impl From<User> for GeneralUserOutput {
    fn from(value: User) -> Self {
        Self {
            id: value.id(),
            email: value.email(),
            username: value.username(),
            profile: GeneralUserProfileOutput::from(value.profile()),
        }
    }
}

impl From<UserProfile> for GeneralUserProfileOutput {
    fn from(value: UserProfile) -> Self {
        Self {
            first_name: value.first_name(),

            last_name: value.last_name(),
            bio: value.bio(),
            phone_numbers: value.phone_numbers(),
            avatar_url: value.avatar_url(),
            addresses: value.addresses(),
            website: value.website(),
        }
    }
}
