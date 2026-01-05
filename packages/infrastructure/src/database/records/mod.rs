use serde::Deserialize;

pub mod report;
pub mod role;
pub mod user;

// 1. A private helper that can be a string OR a Thing map
#[allow(unused)]
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum IdHelper {
    String(String),
    Thing { tb: String, id: String },
}
