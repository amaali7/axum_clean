use crate::{events::DomainEvent, DomainError};

pub mod report;
pub mod role;
pub mod specification;
pub mod user;

pub use report::ReportRepository;
pub use role::RoleRepository;
pub use specification::{AndSpecification, Specification};
pub use user::{EmailService, UserRepository};

pub trait DomainEventPublisher: Send + Sync {
    fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), DomainError>;
}
