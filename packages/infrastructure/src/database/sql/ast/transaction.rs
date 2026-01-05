use super::QueryAst;

#[derive(Debug, Clone)]
pub struct TransactionAst {
    pub queries: Vec<QueryAst>,
}

impl TransactionAst {
    pub fn new() -> Self {
        Self {
            queries: Vec::new(),
        }
    }

    pub fn push(mut self, query: QueryAst) -> Self {
        self.queries.push(query);
        self
    }
}
