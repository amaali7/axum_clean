#[derive(Debug, Clone)]
pub enum PathSegment {
    Field(String),
    Index(usize),
    Wildcard,
}

impl PathSegment {
    pub fn to_surreal(&self) -> String {
        match self {
            PathSegment::Field(f) => f.clone(),
            PathSegment::Index(i) => format!("[{}]", i),
            PathSegment::Wildcard => "*".into(),
        }
    }

    pub fn to_sql(&self) -> String {
        match self {
            PathSegment::Field(f) => f.clone(),
            PathSegment::Index(_) => {
                panic!("SQL engines do not support index traversal")
            }
            PathSegment::Wildcard => "*".into(),
        }
    }
}
