/* =========================
Query AST root
========================= */
#[derive(Debug, Clone)]
pub enum QueryKind {
    Select,
    Update,
    Delete,
    Insert,
}

#[derive(Debug, Clone)]
pub struct QueryAst {
    pub kind: QueryKind,

    /// FROM table
    pub table: Table,

    /// SELECT / RETURN
    pub projection: Projection,

    /// JOINs / graph traversals
    pub joins: Vec<Join>,

    /// WHERE clause
    pub filter: Option<Expr>,

    /// ORDER BY
    pub order_by: Vec<OrderBy>,

    /// GROUP BY
    pub group_by: Vec<FieldPath>,

    /// LIMIT / START
    pub limit: Option<Limit>,

    /// UPDATE / INSERT payload
    pub assignments: Vec<(FieldPath, Bind)>,

    /// Bindings
    pub bindings: Vec<Bind>,
}

#[derive(Debug, Clone, Copy)]
pub enum Edge {
    AssignedTo,
    AuthoredBy,
    ReviewedBy,
    HasRole,
    HasPermission,
}

impl Edge {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AssignedTo => "assigned_to",
            Self::AuthoredBy => "authored_by",
            Self::ReviewedBy => "reviewed_by",
            Self::HasRole => "has_role",
            Self::HasPermission => "has_permission",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Projection {
    /// Explicit fields
    Fields(Vec<FieldPath>),

    /// SELECT *
    All,

    /// Role/permission-based projection
    Conditional {
        condition: Expr,
        fields: Vec<FieldPath>,
    },
}

#[derive(Debug, Clone)]
pub struct OrderBy {
    pub field: FieldPath,
    pub desc: bool,
}

#[derive(Debug, Clone)]
pub struct Limit {
    pub limit: u64,
    pub start: Option<u64>,
}

impl QueryAst {
    pub fn compile(&self) -> (String, Bindings) {
        let mut out = String::new();
        let mut binds = Bindings::new();

        match self.kind {
            QueryKind::Select => out.push_str("SELECT "),
            QueryKind::Update => out.push_str("UPDATE "),
            QueryKind::Delete => out.push_str("DELETE "),
            QueryKind::Insert => out.push_str("INSERT "),
        }

        match &self.projection {
            Projection::All => out.push('*'),
            Projection::Fields(fields) => {
                for (i, f) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    f.compile(&mut out, &mut binds);
                }
            }
            _ => unimplemented!(),
        }

        out.push_str(" FROM ");
        out.push_str(self.table.as_str());

        for join in &self.joins {
            out.push(' ');
            join.compile(&mut out, &mut binds);
        }

        if let Some(filter) = &self.filter {
            out.push_str(" WHERE ");
            filter.compile(&mut out, &mut binds);
        }

        if let Some(limit) = &self.limit {
            out.push_str(" LIMIT ");
            out.push_str(&limit.limit.to_string());
        }

        (out, binds)
    }
}
