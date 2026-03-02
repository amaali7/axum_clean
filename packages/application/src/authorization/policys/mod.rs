use super::{engine::AuthorizationContext, AccessDecision};

pub mod report;
pub trait AuthorizationPolicy {
    fn evaluate(&self, ctx: &AuthorizationContext) -> Option<AccessDecision>;
}
