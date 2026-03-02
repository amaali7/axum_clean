pub mod action;
pub mod attributes;
pub mod engine;
pub mod policys;
pub mod relation;
pub mod resource_type;

#[derive(Debug, Clone)]
pub enum AccessDecision {
    Allow,
    Deny,
}
