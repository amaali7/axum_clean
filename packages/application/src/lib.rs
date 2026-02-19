#![allow(dead_code)]

use domain::{Permission, RoleId, UserId};
pub mod dto;
pub mod error;
pub mod ports;
pub mod usecases;

#[derive(Debug, Clone)]
pub struct RequestContex {
    user_id: UserId,
    permission_set: Vec<Permission>,
    role_set: Vec<RoleId>,
}

impl RequestContex {
    pub fn new(user_id: UserId, permission_set: &[Permission], role_set: &[RoleId]) -> Self {
        Self {
            user_id,
            permission_set: permission_set.to_vec(),
            role_set: role_set.to_vec(),
        }
    }

    pub fn add_role(&mut self, role: RoleId) {
        self.role_set.push(role);
    }

    pub fn add_role_set(&mut self, role_set: &[RoleId]) {
        self.role_set.extend_from_slice(role_set);
    }

    pub fn add_permission(&mut self, permission: Permission) {
        self.permission_set.push(permission);
    }

    pub fn add_permission_set(&mut self, permission_set: &[Permission]) {
        self.permission_set.extend_from_slice(permission_set);
    }

    pub fn user_id(&self) -> UserId {
        self.user_id.clone()
    }

    pub fn permission_set_string(&self) -> String {
        let mut output = String::new();
        for permission in self.permission_set.iter() {
            output.push_str(&format!("{},", permission));
        }
        output.truncate(output.len() - 1);
        output
    }

    pub fn permission_set(&self) -> Vec<Permission> {
        self.permission_set.clone()
    }

    pub fn role_set(&self) -> Vec<RoleId> {
        self.role_set.clone()
    }

    pub fn user_id_as_str(&self) -> String {
        self.user_id.id()
    }
}

pub trait RequestContexCompiler {
    fn user_compile(&self) -> String;
    fn role_compile(&self) -> String;
    fn report_compile(&self) -> String;
}
