use crate::{Specification, User};

use super::HasPermission;

impl Specification<User> for HasPermission {
    fn is_satisfied_by(&self, user: &User) -> bool {
        user.has_permission(&self.resource, &self.action)
    }
}
