use domain::{
    user::{
        preferences::UserPreferencesParts, profile::UserProfileParts, UserParts, UserPreferences,
    },
    User, UserProfile,
};

use crate::{
    authorization::access_descriptor::AccessControl,
    dto::user::view::{UserPreferencesView, UserProfileView, UserView},
};

use super::{UserField, UserPreferencesField, UserProfileField};

pub struct UserProjector;

impl UserProjector {
    pub fn project(user: User, access: &AccessControl<UserField>) -> UserView {
        let allow = |f| access.readable_fields.contains(&f);
        let UserParts {
            id,
            email,
            username,
            profile,
            preferences,
            status,
            failed_logins,
            locked_until,
            last_login,
            version,
        } = user.into_parts();
        UserView {
            id: allow(UserField::Id).then(|| id),

            email: allow(UserField::Email).then(|| email),

            username: allow(UserField::Username).then(|| username),

            profile: Some(UserProfileProjector::project(profile, access)),

            preferences: Some(UserPreferencesProjector::project(preferences, access)),
            status: allow(UserField::Status).then(|| status),

            failed_logins: allow(UserField::FailedLogins)
                .then(|| failed_logins)
                .flatten(),

            locked_until: allow(UserField::LockedUntil)
                .then(|| locked_until)
                .flatten(),

            last_login: allow(UserField::LastLogin).then(|| last_login).flatten(),

            version: allow(UserField::Version).then(|| version),
        }
    }
}

pub struct UserProfileProjector;

impl UserProfileProjector {
    pub fn project(profile: UserProfile, access: &AccessControl<UserField>) -> UserProfileView {
        let parts = profile.into_parts();
        let allow = |f| access.readable_fields.contains(&UserField::Profile(f));

        UserProfileView {
            first_name: allow(UserProfileField::FirstName).then(|| parts.first_name),
            last_name: allow(UserProfileField::LastName).then(|| parts.last_name),

            // Option<T> fields — move directly, no .then() needed for the value itself
            bio: allow(UserProfileField::Bio).then(|| parts.bio).flatten(),
            avatar_url: allow(UserProfileField::AvatarUrl)
                .then(|| parts.avatar_url)
                .flatten(),
            date_of_birth: allow(UserProfileField::DateOfBirth)
                .then(|| parts.date_of_birth)
                .flatten(),
            website: allow(UserProfileField::Website)
                .then(|| parts.website)
                .flatten(),

            // Vec<T> or other non-Option fields
            phone_numbers: allow(UserProfileField::PhoneNumbers).then(|| parts.phone_numbers),
            addressess: allow(UserProfileField::Addressess).then(|| parts.addressess),

            // Primitive/bool fields
            is_deleted: allow(UserProfileField::IsDeleted).then(|| parts.is_deleted),
            created_at: allow(UserProfileField::CreatedAt).then(|| parts.created_at),
            updated_at: allow(UserProfileField::UpdatedAt).then(|| parts.updated_at),
        }
    }
}

pub struct UserPreferencesProjector;

impl UserPreferencesProjector {
    pub fn project(
        pref: UserPreferences,
        access: &AccessControl<UserField>,
    ) -> UserPreferencesView {
        let parts = pref.into_parts();
        let allow = |f| access.readable_fields.contains(&UserField::Preferences(f));

        UserPreferencesView {
            email_notifications: allow(UserPreferencesField::EmailNotifications)
                .then(|| parts.email_notifications),
            push_notifications: allow(UserPreferencesField::PushNotifications)
                .then(|| parts.push_notifications),
            two_factor_auth: allow(UserPreferencesField::TwoFactorAuth)
                .then(|| parts.two_factor_auth),
            language: allow(UserPreferencesField::Language).then(|| parts.language),
        }
    }
}
