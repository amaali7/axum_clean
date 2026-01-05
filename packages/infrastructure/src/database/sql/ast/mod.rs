// ast.rs
pub mod expr;
pub mod join;
pub mod query;
pub mod transaction;

pub use expr::*;
pub use join::*;
pub use query::*;

use std::collections::BTreeMap;
use surrealdb::sql::Value;

pub type Bindings = BTreeMap<String, Value>;

pub trait Compile {
    fn compile(&self, out: &mut String, binds: &mut Bindings);
}
