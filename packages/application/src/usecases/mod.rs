pub mod report;
pub mod role;
pub mod user;

pub mod user_servies {
    use super::user;
    pub use user::{
        create::CreateUserUseCase,
        delete::{
            all::DeleteUserUseCase, permission::RemovePermissionFromUserUseCase,
            role::RemoveRoleFromUserUseCase,
        },
        read::{
            get_by::{
                general::{
                    email::GetUserByEmailGeneralUseCase, id::GetUserByIdGenaralUseCase,
                    username::GetUserByUsernameGeneralUseCase,
                },
                owner::{
                    email::GetUserByEmailOwnerUseCase, id::GetUserByIdOwnerUseCase,
                    username::GetUserByUsernameOwnerUseCase,
                },
                privilege::{
                    email::GetUserByEmailPrivilegeUseCase, id::GetUserByIdPrivilegeUseCase,
                    username::GetUserByUsernamePrivilegeUseCase,
                },
            },
            list::{geneal::ListUserGeneralUseCase, privilege::ListUserPrivilegeUseCase},
        },
        update::{
            all::UpdateUserUseCase, permission::AssignPermissionToUserUseCase,
            role::AssignRoleToUserUseCase,
        },
    };
}

pub mod role_servies {
    use super::role;
    pub use role::{
        create::CreateRoleUseCase,
        delete::{all::DeleteRoleUseCase, permission::RemovePermissionFromRoleUseCase},
        read::get_by::{
            general::{id::GetRoleByIdGenaralUseCase, name::GetRoleByNameGeneralUseCase},
            privilege::{id::GetRoleByIdPrivilegeUseCase, name::GetRoleByNamePrivilegeUseCase},
        },
        update::{all::UpdateRoleUseCase, permission::AssignPermissionForRoleUseCase},
    };
}

pub mod report_servies {
    use super::report;
    pub use report::{
        create::CreateReportUseCase,
        delete::{
            all::DeleteReportUseCase, permission::RemovePermissionFromReportUseCase,
            reviewer::RemoveReviewerFromReportUseCase,
        },
        read::{
            get_by::{
                auther::{
                    auther::GetReportByAutherIdAutherUseCase, id::GetReportByIdAutherUseCase,
                    title::GetReportByTitleAutherUseCase,
                },
                general::{
                    auther::GetRebortByAutherGeneralCase, id::GetReportByIdGenaralUseCase,
                    title::GetReportByTitleGeneralUseCase,
                },
                privilege::{
                    auther::GetReportByAutherPrivilegeUseCase, id::GetReportByIdPrivilegeUseCase,
                    title::GetReportByTitlePrivilegeUseCase,
                },
            },
            list::{
                auther::ListOfReportRequestedByAutherUseCase,
                reviewer::ListOfReportRequestedByReviewerUseCase,
            },
        },
        update::{
            all::UpdateReportUseCase, permission::AssignPermissionForReportUseCase,
            reviewer::AssignReviewerForReportUseCase,
        },
    };
}
