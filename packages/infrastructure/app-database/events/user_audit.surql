DEFINE EVENT OVERWRITE  user_audit ON TABLE user
WHEN true THEN {
    CREATE audit_log SET
        table = \"user\",
        id = $after.id ?? $before.id,
        action = $event,
        user_id = $user_id,
        before = $before,
        after = $after,
        occurred_at = time::now();
};
