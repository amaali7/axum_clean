use serde::{Deserialize, Deserializer};

use crate::serialization::InfrastructureUserId;

use super::IdHelper;

impl<'de> Deserialize<'de> for InfrastructureUserId {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = IdHelper::deserialize(des)?;
        Ok(match helper {
            IdHelper::String(s) => InfrastructureUserId::new(&s),
            IdHelper::Thing { id, .. } => InfrastructureUserId::new(format!("{id}").as_str()),
        })
    }
}
