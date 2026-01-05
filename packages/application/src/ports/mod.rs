pub mod report;
pub mod role;
pub mod user;

use std::fmt;

pub use report::ReportRepository;
pub use role::RoleRepository;
pub use user::UserRepository;

#[derive(Debug, Clone)]
pub enum OrderBy {
    CreatedBy(SortBy),
    CreatedAt(SortBy),
    UpdatedAt(SortBy),
}

impl OrderBy {
    pub fn sort(&self) -> SortBy {
        match self {
            OrderBy::CreatedBy(order_by_sort) => order_by_sort.clone(),
            OrderBy::CreatedAt(order_by_sort) => order_by_sort.clone(),
            OrderBy::UpdatedAt(order_by_sort) => order_by_sort.clone(),
        }
    }

    pub fn result(&self) -> (String, String) {
        match self {
            OrderBy::CreatedBy(order_by_sort) => {
                ("created_by".to_string(), order_by_sort.to_string())
            }
            OrderBy::CreatedAt(order_by_sort) => {
                ("created_at".to_string(), order_by_sort.to_string())
            }
            OrderBy::UpdatedAt(order_by_sort) => {
                ("updated_at".to_string(), order_by_sort.to_string())
            }
        }
    }
}

impl Default for OrderBy {
    fn default() -> Self {
        Self::CreatedAt(SortBy::default())
    }
}

#[derive(Debug, Clone)]
pub enum SortBy {
    Descending,
    Ascending,
    None,
}

impl Default for SortBy {
    fn default() -> Self {
        Self::Descending
    }
}

impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Descending => write!(f, "DESC"),
            Self::Ascending => write!(f, "ASC"),
            Self::None => write!(f, ""),
        }
    }
}
