use std::collections::HashSet;

use domain::{
    user::UserStatus,
    value_objects::{Addressess, Bio, DateTime, PhoneNumbers, Url},
    Email, Name, Password, Permission, RoleId, UserId, Username,
};

/// Owner User Output Data

pub struct CreateUserInput {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub profile: CreateUserProfileInput,
    pub roles: HashSet<RoleId>,
    pub permissions: HashSet<Permission>, // Cached permissions for performance
    pub preferences: CreateUserPreferencesInput,
    pub status: UserStatus,
}

pub enum StatusInput {
    Active,
    Suspended,
    Inactive,
    Banned,
}

pub struct CreateUserPreferencesInput {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub two_factor_auth: bool,
    pub language: String,
}

pub struct CreateUserProfileInput {
    pub first_name: Name,
    pub last_name: Name,
    pub bio: Bio,
    pub password: Password,
    pub phone_numbers: PhoneNumbers,
    pub avatar_url: Url,
    pub date_of_birth: DateTime,
    pub addresses: Addressess,
    pub website: Url,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
