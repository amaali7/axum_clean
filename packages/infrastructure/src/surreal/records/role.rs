use serde::{Deserialize, Deserializer};

use crate::serialization::SerializedRoleId;

use super::IdHelper;

impl<'de> Deserialize<'de> for SerializedRoleId {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = IdHelper::deserialize(des)?;
        Ok(match helper {
            IdHelper::String(s) => SerializedRoleId::new(&s),
            IdHelper::Thing { id, .. } => SerializedRoleId::new(format!("{id}").as_str()),
        })
    }
}
