#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfrastructurePermission {
    // User management
    CreateUser = 1,
    ViewUsers = 2,
    ManageUsers = 4,
    DeleteUsers = 8,

    // Report permissions
    CreateReport = 200,
    ViewOwnReports = 210,
    ViewReports = 220, // View all reports
    EditOwnReports = 230,
    EditReports = 240,    // Edit any report
    ReviewReports = 250,  // Review and add comments
    ApproveReports = 260, // Final approval
    RejectReports = 270,
    ExportReports = 280,
    DeleteReports = 290,

    // System permissions
    ManageSystem = 1100,
    ViewAuditLogs = 1000,
}

impl InfrastructurePermission {
    pub fn weight(&self) -> u16 {
        self.clone() as u16
    }

    pub fn is_sensitive(&self) -> bool {
        matches!(
            self,
            Self::ManageSystem | Self::DeleteUsers | Self::ViewAuditLogs
        )
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::CreateUser => "Create new users",
            Self::ViewUsers => "View user list and profiles",
            Self::ManageUsers => "Manage user roles and permissions",
            Self::DeleteUsers => "Delete users",
            Self::CreateReport => "Create new reports",
            Self::ViewOwnReports => "View own reports",
            Self::ViewReports => "View all reports",
            Self::EditOwnReports => "Edit own reports",
            Self::EditReports => "Edit any report",
            Self::ReviewReports => "Review and comment on reports",
            Self::ApproveReports => "Approve reports",
            Self::RejectReports => "Reject reports",
            Self::ExportReports => "Export reports to external formats",
            Self::DeleteReports => "Delete reports",
            Self::ManageSystem => "Manage system configuration",
            Self::ViewAuditLogs => "View system audit logs",
        }
    }
}

use std::fmt;

use domain::Permission;
use serde::{Deserialize, Serialize};

impl fmt::Display for InfrastructurePermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            InfrastructurePermission::CreateUser => "create_user",
            InfrastructurePermission::ViewUsers => "view_users",
            InfrastructurePermission::ManageUsers => "manage_users",
            InfrastructurePermission::DeleteUsers => "delete_users",

            InfrastructurePermission::CreateReport => "create_report",
            InfrastructurePermission::ViewOwnReports => "view_own_reports",
            InfrastructurePermission::ViewReports => "view_reports",
            InfrastructurePermission::EditOwnReports => "edit_own_reports",
            InfrastructurePermission::EditReports => "edit_reports",
            InfrastructurePermission::ReviewReports => "review_reports",
            InfrastructurePermission::ApproveReports => "approve_reports",
            InfrastructurePermission::RejectReports => "reject_reports",
            InfrastructurePermission::ExportReports => "export_reports",
            InfrastructurePermission::DeleteReports => "delete_reports",

            InfrastructurePermission::ManageSystem => "manage_system",
            InfrastructurePermission::ViewAuditLogs => "view_audit_logs",
        };
        write!(f, "{}", s)
    }
}

impl From<Permission> for InfrastructurePermission {
    fn from(value: Permission) -> Self {
        match value {
            Permission::CreateUser => Self::CreateUser,
            Permission::ViewUsers => Self::ViewUsers,
            Permission::ManageUsers => Self::ManageUsers,
            Permission::DeleteUsers => Self::DeleteUsers,
            Permission::CreateReport => Self::CreateReport,
            Permission::ViewOwnReports => Self::ViewOwnReports,
            Permission::ViewReports => Self::ViewReports,
            Permission::EditOwnReports => Self::EditOwnReports,
            Permission::EditReports => Self::EditReports,
            Permission::ReviewReports => Self::ReviewReports,
            Permission::ApproveReports => Self::ApproveReports,
            Permission::RejectReports => Self::RejectReports,
            Permission::ExportReports => Self::ExportReports,
            Permission::DeleteReports => Self::DeleteReports,
            Permission::ManageSystem => Self::ManageSystem,
            Permission::ViewAuditLogs => Self::ViewAuditLogs,
        }
    }
}

impl From<InfrastructurePermission> for Permission {
    fn from(value: InfrastructurePermission) -> Self {
        match value {
            InfrastructurePermission::CreateUser => Self::CreateUser,
            InfrastructurePermission::ViewUsers => Self::ViewUsers,
            InfrastructurePermission::ManageUsers => Self::ManageUsers,
            InfrastructurePermission::DeleteUsers => Self::DeleteUsers,
            InfrastructurePermission::CreateReport => Self::CreateReport,
            InfrastructurePermission::ViewOwnReports => Self::ViewOwnReports,
            InfrastructurePermission::ViewReports => Self::ViewReports,
            InfrastructurePermission::EditOwnReports => Self::EditOwnReports,
            InfrastructurePermission::EditReports => Self::EditReports,
            InfrastructurePermission::ReviewReports => Self::ReviewReports,
            InfrastructurePermission::ApproveReports => Self::ApproveReports,
            InfrastructurePermission::RejectReports => Self::RejectReports,
            InfrastructurePermission::ExportReports => Self::ExportReports,
            InfrastructurePermission::DeleteReports => Self::DeleteReports,
            InfrastructurePermission::ManageSystem => Self::ManageSystem,
            InfrastructurePermission::ViewAuditLogs => Self::ViewAuditLogs,
        }
    }
}
