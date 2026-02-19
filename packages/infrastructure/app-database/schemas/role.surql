# Define the role table
DEFINE TABLE OVERWRITE role SCHEMALESS;

# ID as record reference
DEFINE FIELD OVERWRITE id ON role TYPE record<role>;

# Basic fields
DEFINE FIELD OVERWRITE name ON role TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE description ON role TYPE string;
DEFINE FIELD OVERWRITE is_system_role ON role TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE created_at ON role TYPE datetime DEFAULT time::now();

# Permissions array with enum validation
DEFINE FIELD OVERWRITE permissions ON role TYPE array<string>;

# Indexes
DEFINE INDEX idx_name ON role FIELDS name UNIQUE;
