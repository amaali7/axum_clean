// builder.rs

use surrealdb::sql::Value;

use super::{
    ast::{Expr, Join, QueryAst, QueryKind},
    schema::{QueryField, Table},
};

#[derive(Default)]
pub struct QueryBuilder {
    ast: QueryAst,
}

impl QueryBuilder {
    pub fn select(table: Table) -> Self {
        Self {
            ast: QueryAst {
                kind: QueryKind::Select,
                table: table.as_str(),
                select: vec![],
                joins: vec![],
                where_: vec![],
                order_by: vec![],
                limit: None,
                start: None,
                bindings: Default::default(),
            },
        }
    }

    pub fn delete(table: Table) -> Self {
        Self {
            ast: QueryAst {
                kind: QueryKind::Delete,
                table: table.as_str(),
                select: vec![],
                joins: vec![],
                where_: vec![],
                order_by: vec![],
                limit: None,
                start: None,
                bindings: Default::default(),
            },
        }
    }

    pub fn update(table: Table) -> Self {
        Self {
            ast: QueryAst {
                kind: QueryKind::Update,
                table: table.as_str(),
                select: vec![],
                joins: vec![],
                where_: vec![],
                order_by: vec![],
                limit: None,
                start: None,
                bindings: Default::default(),
            },
        }
    }

    pub fn field(mut self, field: impl QueryField) -> Self {
        self.ast.select.push(field.as_str());
        self
    }

    pub fn where_eq(mut self, field: impl QueryField, bind: &str, value: impl Into<Value>) -> Self {
        self.ast.where_.push(Expr::Eq(field.as_str(), bind));
        self.ast.bindings.insert(bind.into(), value.into());
        self
    }

    pub fn join_edge(mut self, edge: Edge, target: Table, alias: Option<&str>) -> Self {
        self.ast.joins.push(Join::Edge {
            edge: edge.as_str(),
            target: target.as_str(),
            alias,
        });
        self
    }

    pub fn order_by(mut self, field: impl QueryField) -> Self {
        self.ast.order_by.push(field.as_str());
        self
    }

    pub fn limit(mut self, n: u64) -> Self {
        self.ast.limit = Some(n);
        self
    }

    pub fn build(self) -> QueryAst {
        self.ast
    }
}
