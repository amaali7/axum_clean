use serde::{Deserialize, Deserializer};

use crate::serialization::SerializedReportId;

use super::IdHelper;

impl<'de> Deserialize<'de> for SerializedReportId {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = IdHelper::deserialize(des)?;
        Ok(match helper {
            IdHelper::String(s) => SerializedReportId::new(&s),
            IdHelper::Thing { id, .. } => SerializedReportId::new(format!("{id}").as_str()),
        })
    }
}
