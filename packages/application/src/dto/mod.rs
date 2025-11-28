pub mod input;
pub mod output;

pub use input::{
    report::report::{ReportContentInput, ReportInput, ReportTypeInput, ReviewCommentInput},
    role::role::RoleInput,
    user::{
        create::{CreateUserInput, CreateUserPreferencesInput, CreateUserProfileInput},
        update::{UpdateUserInput, UpdateUserPreferencesInput, UpdateUserProfileInput},
    },
};

pub use output::{
    report::report::{
        GeneralReportContentOutput, GeneralReportOutput, PreivilegeReportContentOutput,
        PreivilegeReportOutput, PreivilegeReviewCommentOutput,
    },
    role::role::{GeneralRoleOutput, PrivilegeRoleOutput},
    user::user::{
        GeneralUserOutput, GeneralUserProfileOutput, OwnerUserOutput, OwnerUserProfileOutput,
        PrivilegeUserOutput, PrivilegeUserProfileOutput, UserPreferencesOutput,
    },
};
