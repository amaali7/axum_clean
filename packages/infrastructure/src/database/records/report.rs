use serde::{Deserialize, Deserializer};

use crate::serialization::InfrastructureReportId;

use super::IdHelper;

impl<'de> Deserialize<'de> for InfrastructureReportId {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = IdHelper::deserialize(des)?;
        Ok(match helper {
            IdHelper::String(s) => InfrastructureReportId::new(&s),
            IdHelper::Thing { id, .. } => InfrastructureReportId::new(format!("{id}").as_str()),
        })
    }
}
