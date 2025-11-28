use std::collections::{HashSet, VecDeque};

use domain::{
    events::DomainEvent,
    user::{UserPreferences, UserStatus},
    value_objects::{Addressess, Bio, DateTime, PhoneNumbers, Url},
    Email, Name, Permission, RoleId, User, UserId, UserProfile, Username,
};

/// Owner User Output Data

pub struct OwnerUserOutput {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub profile: OwnerUserProfileOutput,
    pub roles: HashSet<RoleId>,
    pub permissions: HashSet<Permission>, // Cached permissions for performance
    pub preferences: UserPreferencesOutput,
    pub status: UserStatus,
    pub events: VecDeque<DomainEvent>,
}

pub struct UserPreferencesOutput {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub two_factor_auth: bool,
    pub language: String,
}

pub struct OwnerUserProfileOutput {
    pub first_name: Name,
    pub last_name: Name,
    pub bio: Option<Bio>,
    pub phone_numbers: Option<PhoneNumbers>,
    pub avatar_url: Option<Url>,
    pub date_of_birth: Option<DateTime>,
    pub addresses: Option<Addressess>,
    pub website: Option<Url>,
}

impl From<User> for OwnerUserOutput {
    fn from(value: User) -> Self {
        OwnerUserOutput {
            id: value.id().clone(),
            email: value.email().clone(),
            username: value.username().clone(),
            profile: OwnerUserProfileOutput::from(value.profile().clone()),
            roles: value.roles().clone(),
            permissions: value.permissions().clone(),
            preferences: UserPreferencesOutput::from(value.preferences().clone()),
            status: value.status().clone(),
            events: value.events(),
        }
    }
}

impl From<UserProfile> for OwnerUserProfileOutput {
    fn from(value: UserProfile) -> Self {
        OwnerUserProfileOutput {
            first_name: value.first_name().clone(),
            last_name: value.last_name().clone(),
            bio: value.bio().clone(),
            phone_numbers: value.phone_numbers().clone(),
            avatar_url: value.avatar_url().clone(),
            date_of_birth: value.date_of_birth().clone(),
            addresses: value.addresses().clone(),
            website: value.website().clone(),
        }
    }
}

impl From<UserPreferences> for UserPreferencesOutput {
    fn from(value: UserPreferences) -> Self {
        UserPreferencesOutput {
            email_notifications: value.email_notifications().clone(),
            push_notifications: value.push_notifications().clone(),
            two_factor_auth: value.two_factor_auth().clone(),
            language: value.language().clone(),
        }
    }
}

/// Not owner but have privileges

pub struct PrivilegeUserOutput {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub profile: PrivilegeUserProfileOutput,
    pub roles: HashSet<RoleId>,
    pub permissions: HashSet<Permission>, // Cached permissions for performance
    pub status: UserStatus,
    pub events: VecDeque<DomainEvent>,
}

pub struct PrivilegeUserProfileOutput {
    pub first_name: Name,
    pub last_name: Name,
    pub bio: Option<Bio>,
    pub phone_numbers: Option<PhoneNumbers>,
    pub avatar_url: Option<Url>,
    pub addresses: Option<Addressess>,
    pub website: Option<Url>,
}

impl From<User> for PrivilegeUserOutput {
    fn from(value: User) -> Self {
        Self {
            id: value.id(),
            email: value.email(),
            username: value.username(),
            profile: PrivilegeUserProfileOutput::from(value.profile()),
            roles: value.roles(),
            permissions: value.permissions(),
            status: value.status(),
            events: value.events(),
        }
    }
}

impl From<UserProfile> for PrivilegeUserProfileOutput {
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
