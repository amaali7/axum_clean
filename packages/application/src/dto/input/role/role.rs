use std::collections::HashSet;

use domain::Permission;

pub struct RoleInput {
    name: String,
    description: String,
    permissions: HashSet<Permission>,
    is_system_role: bool,
    created_at: String,
}
