pub mod report;
pub mod role;
pub mod user;

pub use report::ReportEvent;
pub use role::RoleEvent;
pub use user::UserEvent;

pub trait DomainEvent: std::fmt::Debug + Send + Sync {
    fn event_type(&self) -> &'static str;
    fn occurred_at(&self) -> String;
    fn version(&self) -> u64 {
        1
    }
}
