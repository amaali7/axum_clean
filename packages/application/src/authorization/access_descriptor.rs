use std::collections::HashSet;

pub trait AccessableField: Copy + Eq {
    fn db_column(&self) -> &'static str;

    /// nested selection support
    fn sub_fields(&self) -> Option<&[Self]> {
        None
    }
}

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
