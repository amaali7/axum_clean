use domain::traits::field::Field;
// FIXME: tenant_id tybo, updated_at field missing, roles is has_set of roles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipField {
    UserId,
    TenetId,
    Roles,
    CreatedAt,
    Version,
}

impl Field for MembershipField {
    fn name(&self) -> &'static str {
        match self {
            MembershipField::UserId => "user_id",
            MembershipField::TenetId => "tenant_id",
            MembershipField::Roles => "roles",
            MembershipField::CreatedAt => "created_at",
            MembershipField::Version => "version",
        }
    }
}
