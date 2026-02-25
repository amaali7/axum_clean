pub mod request;
pub mod response;

pub mod report_dto {
    pub mod request {
        pub use super::super::request::report::{
            create::{CreateReportContentRequest, CreateReportRequest, CreateReviewCommentRequest},
            update::{UpdateReportContentRequest, UpdateReportRequest, UpdateReviewCommentRequest},
        };
    }
    pub mod response {
        pub use super::super::response::report::{
            auther::{
                AutherReportContentResponse, AutherReportResponse, AutherReviewCommentResponse,
            },
            general::{GeneralReportContentResponse, GeneralReportResponse},
            //             privilege::{
            //                 PreivilegeReportContentResponse, PreivilegeReportResponse,
            //                 PreivilegeReviewCommentResponse,
            //             },
            //             reviewer::{
            //                 ReviewerReportContentResponse, ReviewerReportResponse,
            //                 ReviewerReviewCommentResponse,
            //             },
        };
    }
}
pub mod role_dto {
    pub mod request {
        pub use super::super::request::role::{
            create::CreateRoleRequest, update::UpdateRoleRequest,
        };
    }
    pub mod response {
        pub use super::super::response::role::{
            general::GeneralRoleResponse, privilege::PrivilegeRoleResponse,
        };
    }
}
pub mod user_dto {
    pub mod request {
        pub use super::super::request::user::{
            create::{CreateUserPreferencesRequest, CreateUserProfileRequest, CreateUserRequest},
            update::{UpdateUserPreferencesRequest, UpdateUserProfileRequest, UpdateUserRequest},
        };
    }
    pub mod response {
        pub use super::super::response::user::{
            general::{GeneralUserProfileResponse, GeneralUserResponse},
            owner::{OwnerUserPreferencesResponse, OwnerUserProfileResponse, OwnerUserResponse},
            privilege::{PrivilegeUserProfileResponse, PrivilegeUserResponse},
        };
    }
}
