#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterfacePermission {
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

impl InterfacePermission {
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

impl fmt::Display for InterfacePermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            InterfacePermission::CreateUser => "create_user",
            InterfacePermission::ViewUsers => "view_users",
            InterfacePermission::ManageUsers => "manage_users",
            InterfacePermission::DeleteUsers => "delete_users",

            InterfacePermission::CreateReport => "create_report",
            InterfacePermission::ViewOwnReports => "view_own_reports",
            InterfacePermission::ViewReports => "view_reports",
            InterfacePermission::EditOwnReports => "edit_own_reports",
            InterfacePermission::EditReports => "edit_reports",
            InterfacePermission::ReviewReports => "review_reports",
            InterfacePermission::ApproveReports => "approve_reports",
            InterfacePermission::RejectReports => "reject_reports",
            InterfacePermission::ExportReports => "export_reports",
            InterfacePermission::DeleteReports => "delete_reports",

            InterfacePermission::ManageSystem => "manage_system",
            InterfacePermission::ViewAuditLogs => "view_audit_logs",
        };
        write!(f, "{}", s)
    }
}

impl From<Permission> for InterfacePermission {
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

impl From<InterfacePermission> for Permission {
    fn from(value: InterfacePermission) -> Self {
        match value {
            InterfacePermission::CreateUser => Self::CreateUser,
            InterfacePermission::ViewUsers => Self::ViewUsers,
            InterfacePermission::ManageUsers => Self::ManageUsers,
            InterfacePermission::DeleteUsers => Self::DeleteUsers,
            InterfacePermission::CreateReport => Self::CreateReport,
            InterfacePermission::ViewOwnReports => Self::ViewOwnReports,
            InterfacePermission::ViewReports => Self::ViewReports,
            InterfacePermission::EditOwnReports => Self::EditOwnReports,
            InterfacePermission::EditReports => Self::EditReports,
            InterfacePermission::ReviewReports => Self::ReviewReports,
            InterfacePermission::ApproveReports => Self::ApproveReports,
            InterfacePermission::RejectReports => Self::RejectReports,
            InterfacePermission::ExportReports => Self::ExportReports,
            InterfacePermission::DeleteReports => Self::DeleteReports,
            InterfacePermission::ManageSystem => Self::ManageSystem,
            InterfacePermission::ViewAuditLogs => Self::ViewAuditLogs,
        }
    }
}
