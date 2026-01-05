use serde::{Deserialize, Deserializer};

use crate::serialization::SerializedUserId;

use super::IdHelper;

impl<'de> Deserialize<'de> for SerializedUserId {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = IdHelper::deserialize(des)?;
        Ok(match helper {
            IdHelper::String(s) => SerializedUserId::new(&s),
            IdHelper::Thing { id, .. } => SerializedUserId::new(format!("{id}").as_str()),
        })
    }
}
