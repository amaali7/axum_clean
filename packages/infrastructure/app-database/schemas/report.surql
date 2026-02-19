# Define the report table
DEFINE TABLE OVERWRITE report SCHEMALESS;

# ID as record reference
DEFINE FIELD OVERWRITE id ON report TYPE record<report>;

# Basic metadata
DEFINE FIELD OVERWRITE title ON report TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE report_type ON report TYPE string; 
DEFINE FIELD OVERWRITE status ON report TYPE string 
    DEFAULT 'Draft';

# Permissions array (same enum as role)
DEFINE FIELD OVERWRITE permissions ON report TYPE array<string>;
DEFINE FIELD OVERWRITE permissions.* ON report TYPE string ;

# Author and reviewers
DEFINE FIELD OVERWRITE author_id ON report TYPE record<user> ASSERT $value != NONE;
DEFINE FIELD OVERWRITE assigned_reviewer_id ON report TYPE array<record<user>>;

# Timestamps and versioning
DEFINE FIELD OVERWRITE created_at ON report TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at ON report TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE due_date ON report TYPE option<datetime>;
DEFINE FIELD OVERWRITE version ON report TYPE int DEFAULT 0;

# Nested content object
DEFINE FIELD OVERWRITE content ON report TYPE object;
DEFINE FIELD OVERWRITE content.body ON report TYPE string;
DEFINE FIELD OVERWRITE content.attachments ON report TYPE array<string>;
DEFINE FIELD OVERWRITE content.rejection_reason ON report TYPE option<string>;

# Nested review comments array
DEFINE FIELD OVERWRITE content.review_comments ON report TYPE array;
DEFINE FIELD OVERWRITE content.review_comments.* ON report TYPE object;
DEFINE FIELD OVERWRITE content.review_comments.*.reviewer_id ON report TYPE record<user>;
DEFINE FIELD OVERWRITE content.review_comments.*.comment ON report TYPE string;
DEFINE FIELD OVERWRITE content.review_comments.*.created_at ON report TYPE datetime;
