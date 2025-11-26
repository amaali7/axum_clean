use std::collections::HashSet;

use domain::{
    value_objects::{Addressess, Bio, PhoneNumbers, Url},
    Email, Name, Password, Permission, RoleId, Username,
};

pub struct CreateUserInput {
    email: Email,
    username: Username,
    profile: CreateUserProfileInput,
    roles: HashSet<RoleId>,
    permissions: HashSet<Permission>, // Cached permissions for performance
    preferences: CreateUserPreferencesInput,
    status: UserStatusInput,
}

pub enum UserStatusInput {
    Active,
    Suspended,
    Inactive,
    Banned,
}

pub struct CreateUserPreferencesInput {
    email_notifications: bool,
    push_notifications: bool,
    two_factor_auth: bool,
    language: String,
    timezone: String,
}

pub struct CreateUserProfileInput {
    first_name: Name,
    last_name: Name,
    password: Password,
    bio: Option<Bio>,
    phone_numbers: Option<PhoneNumbers>,
    avatar_url: Option<Url>,
    date_of_birth: Option<NaiveDate>,
    addresses: Option<Addressess>,
    website: Option<Url>,
}
