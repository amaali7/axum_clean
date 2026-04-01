pub mod config;
pub mod environment;
pub mod membership;
pub mod temporary_grant;
use config::TenantConfigField;
pub use membership::MembershipField;

#[derive(Debug, Clone)]
pub enum TenantField {
    Id,
    Name,
    Description,
    CreatedAt,
    Config(TenantConfigField),
    Version,
}
