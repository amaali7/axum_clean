/* =========================
Joins / Graph edges
========================= */

#[derive(Debug, Clone)]
pub enum Join {
    /// ->edge->target
    Edge {
        edge: Edge,    // typed edge enum
        target: Table, // typed table enum
        alias: Option<String>,
    },

    /// <-edge-<target
    ReverseEdge {
        edge: Edge,
        target: Table,
        alias: Option<String>,
    },

    /// FETCH field
    Fetch { field: FieldPath },

    /// LET $alias = (subquery)
    Subquery { alias: String, query: Box<QueryAst> },
}

impl Compile for Join {
    fn compile(&self, out: &mut String, binds: &mut Bindings) {
        match self {
            Join::Edge {
                edge,
                target,
                alias,
            } => {
                out.push_str("->");
                out.push_str(edge.as_str());
                out.push_str("->");
                out.push_str(target.as_str());

                if let Some(a) = alias {
                    out.push_str(" AS ");
                    out.push_str(a);
                }
            }

            Join::Fetch { field } => {
                out.push_str(" FETCH ");
                field.compile(out, binds);
            }

            Join::Subquery { alias, query } => {
                out.push_str(" LET $");
                out.push_str(alias);
                out.push_str(" = ");
                query.compile(out, binds);
            }

            _ => unimplemented!("Join variant not compiled yet"),
        }
    }
}
