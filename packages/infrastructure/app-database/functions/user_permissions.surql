DEFINE FUNCTION fn::user_has_all_permissions($user_id: record<user>, $required: array<string>) {
    # Get user record
    LET $user = (SELECT * FROM ONLY $user_id) ?? THROW "User not found: " + $user_id;
    
    # Collect all permissions from user directly
    LET $user_perms = $user.permissions;
    
    # Collect all permissions from user's roles (handle empty roles)
    LET $role_perms = array::flatten(
        (SELECT permissions FROM $user.roles).permissions ?? []
    );
    
    # Combine and deduplicate all effective permissions
    LET $all_perms = array::distinct(array::concat($user_perms, $role_perms));
    
    # Return true if no missing permissions (difference is empty)
    RETURN array::difference($required, $all_perms) = [];
};
