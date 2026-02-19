# Define the main user table

DEFINE TABLE OVERWRITE user SCHEMALESS;

# Custom types for new ID format
DEFINE FIELD OVERWRITE id ON user TYPE record<user>;
DEFINE FIELD OVERWRITE email ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE username ON user TYPE string ASSERT $value != NONE;

# Profile as embedded object
DEFINE FIELD OVERWRITE profile ON user TYPE object;
DEFINE FIELD OVERWRITE profile.first_name ON user TYPE string;
DEFINE FIELD OVERWRITE profile.last_name ON user TYPE string;
DEFINE FIELD OVERWRITE profile.password ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE profile.bio ON user TYPE option<string>;
DEFINE FIELD OVERWRITE profile.phone_numbers ON user TYPE array;
DEFINE FIELD OVERWRITE profile.avatar_url ON user TYPE option<string>;
DEFINE FIELD OVERWRITE profile.date_of_birth ON user TYPE option<datetime>;
DEFINE FIELD OVERWRITE profile.addresses ON user TYPE array;
DEFINE FIELD OVERWRITE profile.website ON user TYPE option<string>;
DEFINE FIELD OVERWRITE profile.is_deleted ON user TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE profile.created_at ON user TYPE datetime;
DEFINE FIELD OVERWRITE profile.updated_at ON user TYPE datetime;

# Roles as array of record references
DEFINE FIELD OVERWRITE roles ON user TYPE array<record<role>>;
DEFINE FIELD OVERWRITE permissions ON user TYPE array<string>;

# Preferences as embedded object
DEFINE FIELD OVERWRITE preferences ON user TYPE object;
DEFINE FIELD OVERWRITE preferences.email_notifications ON user TYPE bool DEFAULT true;
DEFINE FIELD OVERWRITE preferences.push_notifications ON user TYPE bool DEFAULT true;
DEFINE FIELD OVERWRITE preferences.two_factor_auth ON user TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE preferences.language ON user TYPE string DEFAULT 'en';

# Status enum
DEFINE FIELD OVERWRITE status ON user TYPE string DEFAULT 'Inactive';

# Indexes for common queries
DEFINE INDEX idx_email ON user FIELDS email UNIQUE;
DEFINE INDEX idx_username ON user FIELDS username UNIQUE;
DEFINE INDEX idx_status ON user FIELDS status;
