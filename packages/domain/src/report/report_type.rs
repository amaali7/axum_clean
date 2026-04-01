use crate::{DateTime, Description, Name};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ReportTypeId(String);

impl ReportTypeId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
    pub fn id(&self) -> &String {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for ReportTypeId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ReportTypeId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl std::fmt::Display for ReportTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Hash, Clone, PartialEq, Eq)]
pub struct ReportType {
    id: ReportTypeId,
    name: Name,
    description: Description,
    created_at: DateTime,
}

#[derive(Debug)]
pub struct ReportTypeParts {
    pub id: ReportTypeId,
    pub name: Name,
    pub description: Description,
    pub created_at: DateTime,
}

impl ReportType {
    pub fn new(
        id: ReportTypeId,
        name: Name,
        description: Description,
        created_at: DateTime,
    ) -> Self {
        Self {
            id,
            name,
            description,
            created_at,
        }
    }

    pub fn into_parts(self) -> ReportTypeParts {
        let Self {
            id,
            name,
            description,
            created_at,
        } = self;
        ReportTypeParts {
            id,
            name,
            description,
            created_at,
        }
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }
}
