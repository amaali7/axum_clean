/* =========================
Expressions
========================= */

use super::Compile;

#[derive(Debug, Clone)]
pub enum Expr {
    /* =========================
    Comparison
    ========================= */
    Eq(FieldPath, Bind),
    Ne(FieldPath, Bind),
    Gt(FieldPath, Bind),
    Gte(FieldPath, Bind),
    Lt(FieldPath, Bind),
    Lte(FieldPath, Bind),

    /* =========================
    Set / collection
    ========================= */
    In(FieldPath, Bind),
    NotIn(FieldPath, Bind),
    Contains(FieldPath, Bind),    // array contains value
    ContainsAny(FieldPath, Bind), // array contains any
    ContainsAll(FieldPath, Bind), // array contains all

    /* =========================
    String operations
    ========================= */
    Like(FieldPath, Bind),
    ILike(FieldPath, Bind),
    StartsWith(FieldPath, Bind),
    EndsWith(FieldPath, Bind),
    Matches(FieldPath, Bind), // regex

    /* =========================
    Null / existence
    ========================= */
    IsNull(FieldPath),
    IsNotNull(FieldPath),
    Exists(FieldPath),
    NotExists(FieldPath),

    /* =========================
    Boolean logic
    ========================= */
    And(Vec<Expr>),
    Or(Vec<Expr>),
    Not(Box<Expr>),

    /* =========================
    Permission-aware helpers
    ========================= */
    HasPermission {
        field: FieldPath, // permissions field
        permission: Bind,
    },

    /* =========================
    Subquery / advanced
    ========================= */
    Subquery {
        field: FieldPath,
        query: Box<QueryAst>,
    },

    Raw(String), // escape hatch (discouraged but necessary)
}

impl Compile for Expr {
    fn compile(&self, out: &mut String, binds: &mut Bindings) {
        match self {
            Expr::Eq(path, bind) => {
                path.compile(out, binds);
                out.push_str(" = $");
                out.push_str(&bind.name);
                binds.insert(bind.name.clone(), bind.value.clone());
            }

            Expr::Contains(path, bind) => {
                path.compile(out, binds);
                out.push_str(" CONTAINS $");
                out.push_str(&bind.name);
                binds.insert(bind.name.clone(), bind.value.clone());
            }

            Expr::And(exprs) => {
                out.push('(');
                for (i, e) in exprs.iter().enumerate() {
                    if i > 0 {
                        out.push_str(" AND ");
                    }
                    e.compile(out, binds);
                }
                out.push(')');
            }

            Expr::Or(exprs) => {
                out.push('(');
                for (i, e) in exprs.iter().enumerate() {
                    if i > 0 {
                        out.push_str(" OR ");
                    }
                    e.compile(out, binds);
                }
                out.push(')');
            }

            Expr::Not(expr) => {
                out.push_str("NOT ");
                expr.compile(out, binds);
            }

            Expr::IsNull(path) => {
                path.compile(out, binds);
                out.push_str(" IS NONE");
            }

            Expr::Raw(s) => {
                out.push_str(s);
            }

            _ => unimplemented!("Expr variant not compiled yet"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetExpr {
    pub field: Vec<PathSegment>,
    pub bind: String,
}
