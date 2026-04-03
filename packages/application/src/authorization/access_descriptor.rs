use std::collections::HashSet;

pub trait AccessableField {}

pub struct AccessControl<T: AccessableField> {
    pub can_read: bool,
    pub readable_fields: HashSet<T>,
    pub writable_fields: HashSet<T>,
}

impl<T: AccessableField> AccessControl<T> {
    pub fn new(can_read: bool, readable_fields: HashSet<T>, writable_fields: HashSet<T>) -> Self {
        Self {
            can_read,
            readable_fields,
            writable_fields,
        }
    }
}
