DEFINE EVENT OVERWRITE  role_audit ON TABLE role
WHEN true THEN {
    CREATE audit_log SET
        table = \"role\",
        id = $after.id ?? $before.id,
        action = $event,
        user_id = $user_id,
        before = $before,
        after = $after,
        occurred_at = time::now();
};
