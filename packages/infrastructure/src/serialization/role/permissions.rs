#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SerializedPermission {
    // User management
    CreateUser,
    ViewUsers,
    ManageUsers,
    DeleteUsers,

    // Report permissions
    CreateReport,
    ViewOwnReports,
    ViewReports, // View all reports
    EditOwnReports,
    EditReports,    // Edit any report
    ReviewReports,  // Review and add comments
    ApproveReports, // Final approval
    RejectReports,
    ExportReports,
    DeleteReports,

    // System permissions
    ManageSystem,
    ViewAuditLogs,
}

impl SerializedPermission {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CreateUser,
            Self::ViewUsers,
            Self::ManageUsers,
            Self::DeleteUsers,
            Self::CreateReport,
            Self::ViewOwnReports,
            Self::ViewReports,
            Self::EditOwnReports,
            Self::EditReports,
            Self::ReviewReports,
            Self::ApproveReports,
            Self::RejectReports,
            Self::ExportReports,
            Self::DeleteReports,
            Self::ManageSystem,
            Self::ViewAuditLogs,
        ]
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

impl fmt::Display for SerializedPermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SerializedPermission::CreateUser => "create_user",
            SerializedPermission::ViewUsers => "view_users",
            SerializedPermission::ManageUsers => "manage_users",
            SerializedPermission::DeleteUsers => "delete_users",

            SerializedPermission::CreateReport => "create_report",
            SerializedPermission::ViewOwnReports => "view_own_reports",
            SerializedPermission::ViewReports => "view_reports",
            SerializedPermission::EditOwnReports => "edit_own_reports",
            SerializedPermission::EditReports => "edit_reports",
            SerializedPermission::ReviewReports => "review_reports",
            SerializedPermission::ApproveReports => "approve_reports",
            SerializedPermission::RejectReports => "reject_reports",
            SerializedPermission::ExportReports => "export_reports",
            SerializedPermission::DeleteReports => "delete_reports",

            SerializedPermission::ManageSystem => "manage_system",
            SerializedPermission::ViewAuditLogs => "view_audit_logs",
        };
        write!(f, "{}", s)
    }
}

impl From<Permission> for SerializedPermission {
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

impl From<SerializedPermission> for Permission {
    fn from(value: SerializedPermission) -> Self {
        match value {
            SerializedPermission::CreateUser => Self::CreateUser,
            SerializedPermission::ViewUsers => Self::ViewUsers,
            SerializedPermission::ManageUsers => Self::ManageUsers,
            SerializedPermission::DeleteUsers => Self::DeleteUsers,
            SerializedPermission::CreateReport => Self::CreateReport,
            SerializedPermission::ViewOwnReports => Self::ViewOwnReports,
            SerializedPermission::ViewReports => Self::ViewReports,
            SerializedPermission::EditOwnReports => Self::EditOwnReports,
            SerializedPermission::EditReports => Self::EditReports,
            SerializedPermission::ReviewReports => Self::ReviewReports,
            SerializedPermission::ApproveReports => Self::ApproveReports,
            SerializedPermission::RejectReports => Self::RejectReports,
            SerializedPermission::ExportReports => Self::ExportReports,
            SerializedPermission::DeleteReports => Self::DeleteReports,
            SerializedPermission::ManageSystem => Self::ManageSystem,
            SerializedPermission::ViewAuditLogs => Self::ViewAuditLogs,
        }
    }
}
