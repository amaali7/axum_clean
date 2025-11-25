#![allow(dead_code)]
pub mod error;
pub mod events;
pub mod report;
pub mod role;
pub mod traits;
pub mod user;
pub mod value_objects;

// Re-export main types
pub use error::DomainError;
pub use report::{Report, ReportContent, ReportId, ReportStatus, ReportType};
pub use role::{Permission, Role, RoleId};
pub use traits::{ReportRepository, RoleRepository, UserRepository};
pub use user::{User, UserId, UserProfile};
pub use value_objects::{Email, Name, Password, PhoneNumber, Username};
