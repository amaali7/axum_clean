use super::{DatabaseEngine, PathSegment};

pub type FieldPath = Vec<PathSegment>;

impl FieldPath {
    pub fn to_string(&self, engine: DatabaseEngine) -> String {
        match engine {
            DatabaseEngine::SurrealDb => self
                .iter()
                .map(|s| s.to_surreal())
                .collect::<Vec<_>>()
                .join("."),

            DatabaseEngine::Sqlx(_) => self
                .iter()
                .map(|s| s.to_sql())
                .collect::<Vec<_>>()
                .join("."),
        }
    }
}
