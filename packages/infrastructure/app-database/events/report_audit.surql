DEFINE EVENT OVERWRITE  report_audit ON TABLE report
WHEN true THEN {
    CREATE audit_log SET
        table = \"report\",
        id = $after.id ?? $before.id,
        action = $event,
        user_id = $user_id,
        before = $before,
        after = $after,
        occurred_at = time::now();
};
