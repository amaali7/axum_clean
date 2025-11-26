use std::collections::HashSet;

use domain::{
    value_objects::{Addressess, Bio, PhoneNumbers, Url},
    Email, Name, Password, Permission, RoleId, Username,
};

pub struct UpdateUserInput {
    email: Option<Email>,
    username: Option<Username>,
    profile: Option<UpdateUserProfileInput>,
    roles: Option<HashSet<RoleId>>,
    permissions: Option<HashSet<Permission>>, // Cached permissions for performance
    preferences: Option<UpdateUserPreferencesInput>,
    status: Option<UserStatusInput>,
}

pub enum UserStatusInput {
    Active,
    Suspended,
    Inactive,
    Banned,
}

pub struct UpdateUserPreferencesInput {
    email_notifications: Option<bool>,
    push_notifications: Option<bool>,
    two_factor_auth: Option<bool>,
    language: Option<String>,
    timezone: Option<String>,
}

pub struct UpdateUserProfileInput {
    first_name: Option<Name>,
    last_name: Option<Name>,
    password: Option<Password>,
    bio: Option<Bio>,
    phone_numbers: Option<PhoneNumbers>,
    avatar_url: Option<Url>,
    date_of_birth: Option<NaiveDate>,
    addresses: Option<Addressess>,
    website: Option<Url>,
}
