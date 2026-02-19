DEFINE FUNCTION fn::user_has_all_permissions($report_id: record<report>, $required: array<string>) {
    # Get report record
    LET $report = (SELECT * FROM ONLY $report_id) ?? THROW "Report not found: " + $report_id;
    
    # Collect all permissions from report directly
    LET $report_perms = $report.permissions;
    
    # Return true if no missing permissions (difference is empty)
    RETURN array::difference($required, $report_perms) = [];
};
