pub mod events;
pub mod report;
pub mod role;
pub mod user;

pub use report::ReportRepository;
pub use role::RoleRepository;
pub use user::UserRepository;

#[derive(Debug, Clone)]
pub enum SortBy {
    Descending(String),
    Ascending(String),
    None,
}

impl SortBy {
    pub fn as_string(&self) -> String {
        match self {
            Self::Descending(field) => format!(" {} DESC ", field),
            Self::Ascending(field) => format!(" {} ASC ", field),
            Self::None => format!(""),
        }
    }
}
