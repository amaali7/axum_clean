use chrono::{DateTime, Utc};
use domain::{value_objects::password, Address, Password, UserProfile};
// serializers/user_profile_serializer.rs
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileRecord {
    first_name: String,
    last_name: String,
    password: String,
    bio: Option<String>,
    phone_numbers: Vec<String>,
    avatar_url: Option<String>,
    date_of_birth: Option<DateTime>,
    addresses: Vec<AddressRecord>,
    website: Option<String>,
    is_deleted: bool,
    created_at: DateTime,
    updated_at: DateTime,
}

impl From<UserProfile> for UserProfileRecord {
    fn from(profile: UserProfile) -> Self {
        Self {
            first_name: profile.first_name().to_string(),
            last_name: profile.last_name().to_string(),
            password: match profile.password() {
                Password::Hashed(hashed_password) => hashed_password.as_str().to_string(),
                Password::NoneHashed(none_hashed_password) => {
                    none_hashed_password.as_str().to_string()
                }
            },
            bio: profile.bio().to_string(),
            phone_numbers: ,
            avatar_url: todo!(),
            date_of_birth: todo!(),
            addresses: todo!(),
            website: todo!(),
            is_deleted: todo!(),
            created_at: todo!(),
            updated_at: todo!(),
        }
    }
}

impl TryFrom<UserProfileRecord> for UserProfile {
    type Error = anyhow::Error;

    fn try_from(value: UserProfileRecord) -> Result<Self, Self::Error> {
        let mut profile = UserProfile::new()
            .set_public_profile(value.public_profile)
            .set_show_online_status(value.show_online_status);

        if let Some(display_name) = value.display_name {
            profile = profile.with_display_name(DisplayName::new(&display_name)?);
        }

        if let Some(bio) = value.bio {
            profile = profile.with_bio(Bio::new(&bio)?);
        }

        if let Some(location) = value.location {
            profile = profile.with_location(Location::new(&location)?);
        }

        if let Some(picture) = value.profile_picture {
            profile = profile.with_profile_picture(ProfilePicture::new(&picture)?);
        }

        if let Some(timezone) = value.timezone {
            profile = profile.with_timezone(TimeZone::new(&timezone)?);
        }

        if let Some(website) = value.website_url {
            profile = profile.with_website_url(Url::new(&website)?);
        }

        if let Some(twitter) = value.twitter_url {
            profile = profile.with_twitter_url(Url::new(&twitter)?);
        }

        if let Some(github) = value.github_url {
            profile = profile.with_github_url(Url::new(&github)?);
        }

        if let Some(linkedin) = value.linkedin_url {
            profile = profile.with_linkedin_url(Url::new(&linkedin)?);
        }

        for skill in value.skills {
            profile = profile.add_skill(skill);
        }

        for interest in value.interests {
            profile = profile.add_interest(interest);
        }

        // Note: We can't directly set timestamps since they're private
        // In practice, you'd need a different approach for loading from DB

        Ok(profile)
    }
}
