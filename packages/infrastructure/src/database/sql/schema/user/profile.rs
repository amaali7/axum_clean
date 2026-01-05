use crate::surreal::sql::schema::{FieldPath, PathSegment, QueryField};

#[derive(Debug, Clone)]
pub enum UserProfileField {
    Firstname,
    Lastname,
    Password,
    Bio,
    PhoneNumbers,
    AvatarUrl,
    DateOfBirth,
    Addresses,
    Website,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}

impl QueryField for UserProfileField {
    fn table_ref(&self) -> &str {
        "user"
    }

    fn path(&self) -> FieldPath {
        vec![PathSegment::Field(
            match self {
                Self::Firstname => "first_name",
                Self::Lastname => "last_name",
                Self::Password => "password",
                Self::Bio => "bio",
                Self::PhoneNumbers => "phone_numbers",
                Self::AvatarUrl => "avatar_url",
                Self::DateOfBirth => "date_of_birth",
                Self::Addresses => "addressess",
                Self::Website => "website",
                Self::IsDeleted => "is_deleted",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "update_at",
            }
            .into(),
        )]
    }
}

#[derive(Debug, Clone)]
pub enum AddressField {
    Title,
    Street,
    City,
    State,
    PostalCode,
    Country,
}

impl QueryField for AddressField {
    fn table_ref(&self) -> &str {
        "user"
    }

    fn path(&self) -> FieldPath {
        vec![PathSegment::Field(
            match self {
                Self::Title => "title",
                Self::Street => "street",
                Self::City => "city",
                Self::State => "state",
                Self::PostalCode => "postal_code",
                Self::Country => "country",
            }
            .into(),
        )]
    }
}

/* =========================
Typed traversal helpers
========================= */

/// user.profile
pub struct ProfilePath {
    base: FieldPath,
}

impl UserField {
    pub fn profile(self) -> ProfilePath {
        ProfilePath { base: self.path() }
    }
}

impl ProfilePath {
    /// profile.addresses
    pub fn addresses(self) -> AddressListPath {
        let mut base = self.base;
        base.push(PathSegment::Field("addresses".into()));
        AddressListPath { base }
    }
}

/// profile.addresses (Vec<Address>)
pub struct AddressListPath {
    base: FieldPath,
}

impl AddressListPath {
    /// profile.addresses[*]
    pub fn any(self) -> AddressItemPath {
        let mut base = self.base;
        base.push(PathSegment::All);
        AddressItemPath { base }
    }
}

/// profile.addresses[*]
pub struct AddressItemPath {
    base: FieldPath,
}

impl AddressItemPath {
    /// profile.addresses[*].city
    pub fn city(self) -> FieldPath {
        let mut base = self.base;
        base.push(PathSegment::Field("city".into()));
        base
    }

    pub fn field(self, field: AddressField) -> FieldPath {
        let mut base = self.base;
        base.extend(field.path());
        base
    }
}
