#![allow(dead_code)]

use domain::{Role, TemporaryGrant, TenantId, UserId};
pub mod authorization;
pub mod dto;
pub mod error;
// pub mod ports;
pub mod projection;
pub mod usecases;

#[derive(Debug, Clone)]
pub struct SubjectContex {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub roles: Vec<Role>,
    pub temporary_grants: Vec<TemporaryGrant>,
}

impl SubjectContex {
    pub fn new(
        user_id: UserId,
        tenant_id: TenantId,
        role_set: &[Role],
        temporary_grants: &[TemporaryGrant],
    ) -> Self {
        Self {
            user_id,
            tenant_id,
            roles: role_set.to_vec(),
            temporary_grants: temporary_grants.to_vec(),
        }
    }

    pub fn add_role(&mut self, role: Role) {
        self.roles.push(role);
    }

    pub fn add_roles(&mut self, roles: &[Role]) {
        self.roles.extend_from_slice(roles);
    }

    pub fn add_temporary_grant(&mut self, temporary_grant: TemporaryGrant) {
        self.temporary_grants.push(temporary_grant);
    }

    pub fn add_temporary_grants(&mut self, temporary_grant: &[TemporaryGrant]) {
        self.temporary_grants.extend_from_slice(temporary_grant);
    }

    pub fn user_id(&self) -> UserId {
        self.user_id.clone()
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id.clone()
    }

    pub fn roles(&self) -> Vec<Role> {
        self.roles.clone()
    }

    pub fn temporary_grants(&self) -> Vec<TemporaryGrant> {
        self.temporary_grants.clone()
    }
}
