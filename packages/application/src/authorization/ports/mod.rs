use super::{
    engine::{AuthorizationContext, AuthorizationEngine},
    AccessDecision,
};

pub trait AuthorizationService {
    fn authorize(&self, ctx: &AuthorizationContext) -> AccessDecision;
}

impl AuthorizationService for AuthorizationEngine {
    fn authorize(&self, ctx: &AuthorizationContext) -> AccessDecision {
        self.evaluate(ctx)
    }
}
