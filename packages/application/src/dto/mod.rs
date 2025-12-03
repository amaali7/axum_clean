pub mod input;
pub mod output;

pub mod report_dto {
    pub mod input {
        pub use super::super::input::report::{
            create::{CreateReportContentInput, CreateReportInput, CreateReviewCommentInput},
            update::{UpdateReportContentInput, UpdateReportInput, UpdateReviewCommentInput},
        };
    }
    pub mod output {
        pub use super::super::output::report::{
            auther::{AutherReportContentOutput, AutherReportOutput, AutherReviewCommentOutput},
            general::{GeneralReportContentOutput, GeneralReportOutput},
            privilege::{
                PreivilegeReportContentOutput, PreivilegeReportOutput,
                PreivilegeReviewCommentOutput,
            },
            reviewer::{
                ReviewerReportContentOutput, ReviewerReportOutput, ReviewerReviewCommentOutput,
            },
        };
    }
}
pub mod role_dto {
    pub mod input {
        pub use super::super::input::role::{create::CreateRoleInput, update::UpdateRoleInput};
    }
    pub mod output {

        pub use super::super::output::role::{
            general::GeneralRoleOutput, privilege::PrivilegeRoleOutput,
        };
    }
}
pub mod user_dto {
    pub mod input {
        pub use super::super::input::user::{
            create::{CreateUserInput, CreateUserPreferencesInput, CreateUserProfileInput},
            update::{UpdateUserInput, UpdateUserPreferencesInput, UpdateUserProfileInput},
        };
    }
    pub mod output {
        pub use super::super::output::user::{
            general::{GeneralUserOutput, GeneralUserProfileOutput},
            owner::{OwnerUserOutput, OwnerUserPreferencesOutput, OwnerUserProfileOutput},
            privilege::{PrivilegeUserOutput, PrivilegeUserProfileOutput},
        };
    }
}
