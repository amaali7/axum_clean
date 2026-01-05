use super::compile_ctx::CompileCtx;
use crate::schema::{DatabaseEngine, FieldPath};

#[derive(Debug, Clone)]
pub enum Expr {
    Eq(FieldPath, String),
    In(FieldPath, Vec<String>),
}

impl Expr {
    pub fn compile(&self, ctx: &mut CompileCtx) -> String {
        match self {
            Expr::Eq(path, value) => match ctx.engine {
                DatabaseEngine::SurrealDb => {
                    format!("{} = {}", path.to_string(ctx.engine), value)
                }
                DatabaseEngine::Sqlx(_) => {
                    ctx.bind_counter += 1;
                    format!("{} = ?", path.to_string(ctx.engine))
                }
            },

            Expr::In(path, values) => match ctx.engine {
                DatabaseEngine::SurrealDb => {
                    format!("{} IN [{}]", path.to_string(ctx.engine), values.join(", "))
                }
                DatabaseEngine::Sqlx(_) => {
                    ctx.bind_counter += 1;
                    format!("{} IN (?)", path.to_string(ctx.engine))
                }
            },
        }
    }
}
