pub mod engine;
pub mod field_path;
pub mod path_segment;
pub mod query_field;
pub mod report;
pub mod role;
pub mod user;

pub use engine::{DatabaseEngine, SqlxEngine};
pub use path_segment::PathSegment;
pub use query_field::QueryField;
pub use report::{ReportContentField, ReportField, ReviewCommentFiled};
pub use role::RoleField;
pub use user::{UserField, UserPreferencesField, UserProfileField};

use super::ast::Compile;

#[derive(Debug, Clone, Copy)]
pub enum Table {
    User,
    Role,
    Report,
}

impl Table {
    pub fn as_str(&'a self) -> &'a str {
        match self {
            Table::User => "user",
            Table::Role => "role",
            Table::Report => "report",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Edge {
    HasRole,
    Events,
    Auther,
    AssignedReviewer,
}

impl Edge {
    pub fn as_str(&'a self) -> &'a str {
        match self {
            Self::HasRole => "roles",
            Self::Events => "events",
            Self::Auther => "auther_id",
            Self::AssignedReviewer => "assigned_reviewer_id",
        }
    }
}

pub type FieldPath = Vec<PathSegment>;

impl Compile for FieldPath {
    fn compile(&self, out: &mut String, binds: &mut Bindings) {
        let mut first = true;
        for seg in self {
            if !first {
                out.push('.');
            }
            seg.compile(out, binds);
            first = false;
        }
    }
}
