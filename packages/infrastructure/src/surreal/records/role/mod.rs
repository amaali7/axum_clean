use serde::{Deserialize, Serialize};
use surrealdb::{sql::Id, RecordId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleRecordId(RecordId);

impl RoleRecordId {
    // Create a new RoleRecordId with a UUID
    pub fn new() -> Self {
        RoleRecordId(RecordId::from(("user", Id::uuid())))
    }

    // Create from existing RecordId
    pub fn from_record_id(id: RecordId) -> Self {
        RoleRecordId(id)
    }

    // Get the inner RecordId for database operations
    pub fn as_record_id(&self) -> &RecordId {
        &self.0
    }

    // Get as a Thing (common SurrealDB type)
    pub fn as_thing(&self) -> Thing {
        Thing::from(&self.0)
    }
}

// Implement Serialize and Deserialize for your type
impl Serialize for RoleRecordId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RoleRecordId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let record_id = RecordId::deserialize(deserializer)?;
        Ok(RoleRecordId(record_id))
    }
}
