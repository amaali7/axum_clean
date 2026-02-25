use serde::{Deserialize, Deserializer};

use crate::serialization::InfrastructureRoleId;

use super::IdHelper;

impl<'de> Deserialize<'de> for InfrastructureRoleId {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = IdHelper::deserialize(des)?;
        Ok(match helper {
            IdHelper::String(s) => InfrastructureRoleId::new(&s),
            IdHelper::Thing { id, .. } => InfrastructureRoleId::new(format!("{id}").as_str()),
        })
    }
}
