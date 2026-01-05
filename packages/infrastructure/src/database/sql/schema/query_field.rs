use super::{DatabaseEngine, FieldPath};

pub trait QueryField {
    fn path(&self) -> FieldPath;
    fn table_ref(&self, engine: DatabaseEngine) -> &str;
}
