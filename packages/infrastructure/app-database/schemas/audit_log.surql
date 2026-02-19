DEFINE TABLE OVERWRITE audit_log SCHEMALESS;


DEFINE FIELD OVERWRITE id ON audit_log TYPE record;
DEFINE FIELD OVERWRITE table ON audit_log TYPE string;
DEFINE FIELD OVERWRITE action ON audit_log TYPE string;
DEFINE FIELD OVERWRITE user_id ON audit_log TYPE record<user>;
DEFINE FIELD OVERWRITE before ON audit_log FLEXIBLE TYPE object;
DEFINE FIELD OVERWRITE after ON audit_log FLEXIBLE TYPE object;
DEFINE FIELD OVERWRITE occurred_at ON audit_log TYPE datetime;
