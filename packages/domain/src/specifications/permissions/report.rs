use crate::{Report, Specification};

use super::HasPermission;

impl Specification<Report> for HasPermission {
    fn is_satisfied_by(&self, report: &Report) -> bool {
        report.has_permission(&self.resource, &self.action)
    }
}
