#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
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

impl Permission {
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

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Permission::CreateUser => "create_user",
            Permission::ViewUsers => "view_users",
            Permission::ManageUsers => "manage_users",
            Permission::DeleteUsers => "delete_users",

            Permission::CreateReport => "create_report",
            Permission::ViewOwnReports => "view_own_reports",
            Permission::ViewReports => "view_reports",
            Permission::EditOwnReports => "edit_own_reports",
            Permission::EditReports => "edit_reports",
            Permission::ReviewReports => "review_reports",
            Permission::ApproveReports => "approve_reports",
            Permission::RejectReports => "reject_reports",
            Permission::ExportReports => "export_reports",
            Permission::DeleteReports => "delete_reports",

            Permission::ManageSystem => "manage_system",
            Permission::ViewAuditLogs => "view_audit_logs",
        };
        write!(f, "{}", s)
    }
}
