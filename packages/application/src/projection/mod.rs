pub mod fields;
pub mod projection_any;
pub mod traits;
pub mod vistor;

use std::collections::HashMap;

use domain::traits::field::Field;
use projection_any::ProjectionAny;
use traits::{node::ProjectionNode, projectable::Projectable, relation::Relation};
use vistor::ProjectionVisitor;

pub struct Projection<E: Projectable> {
    fields: Vec<E::Field>,
    relations: HashMap<E::Relation, ProjectionAny>,
}

impl<P: Projectable> ProjectionNode for Projection<P> {
    fn accept(&self, visitor: &mut dyn ProjectionVisitor) {
        visitor.enter_entity(std::any::type_name::<P>());

        for f in &self.fields {
            visitor.visit_field(f.name());
        }

        for (rel, proj) in &self.relations {
            visitor.enter_relation(rel.name());
            proj.accept(visitor);
            visitor.exit_relation();
        }

        visitor.exit_entity();
    }
}

impl<P: Projectable> Projection<P> {
    pub fn new() -> Self {
        Self {
            fields: vec![],
            relations: HashMap::new(),
        }
    }

    pub fn include(mut self, field: P::Field) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with<R>(mut self, relation: P::Relation, projection: Projection<R::Target>) -> Self
    where
        P::Relation: Relation<Target = R::Target>,
        R: Relation,
    {
        self.relations
            .insert(relation, ProjectionAny::new(projection));

        self
    }
}
