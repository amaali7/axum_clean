#![allow(dead_code)]
pub mod error;
pub mod events;
pub mod membership;
pub mod permissions;
pub mod report;
pub mod role;
pub mod shared;
pub mod specifications;
pub mod temporary_grant;
pub mod tenant;
pub mod traits;
pub mod user;
pub mod value_objects;

// Re-export main types
pub use error::DomainError;
pub use events::{DomainEvent, DomainEventId, Event, Table};
pub use membership::{Membership, MembershipParts};
pub use permissions::{Permission, PermissionId, PermissionParts};
pub use report::{Report, ReportContent, ReportId, ReportStatus, ReportType};
pub use role::{Role, RoleId};
pub use shared::shared_str::SharedStr;
pub use temporary_grant::{TemporaryGrant, TemporaryGrantParts};
pub use tenant::{Tenant, TenantId};
pub use traits::{AndSpecification, Specification};
pub use user::{User, UserId, UserProfile};
pub use value_objects::{
    Address, Addressess, Bio, Body, Comment, DateTime, Description, Email, HashedPassword, Name,
    Password, PhoneNumber, PhoneNumbers, Title, Url, Username,
};
