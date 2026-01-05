use crate::schema::engine::DatabaseEngine;

#[derive(Debug, Clone)]
pub struct CompileCtx {
    pub engine: DatabaseEngine,
    pub bind_counter: usize,
}
