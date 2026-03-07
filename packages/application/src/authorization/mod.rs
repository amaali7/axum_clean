pub mod action;
pub mod attributes;
pub mod engine;
pub mod guard;
pub mod policys;
pub mod ports;
pub mod relation;
pub mod resource_type;

#[derive(Debug, Clone)]
pub enum AccessDecision {
    Allow,
    Deny,
}
