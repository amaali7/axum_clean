use crate::SubjectContext;

use super::{
    action::AuthorizationAction, attributes::AuthorizationAttributes, policys::AuthorizationPolicy,
    relation::AuthorizationRelations, resource_type::AuthorizationResourceType, AccessDecision,
};

pub struct AuthorizationContext<'a> {
    pub subject: &'a SubjectContext,
    pub action: AuthorizationAction,

    pub resource_type: AuthorizationResourceType,

    pub subject_attributes: AuthorizationAttributes,
    pub resource_attributes: AuthorizationAttributes,

    pub relations: AuthorizationRelations,
}

pub struct AuthorizationEngine {
    policies: Vec<Box<dyn AuthorizationPolicy>>,
}

impl AuthorizationEngine {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    pub fn register_policy<P>(&mut self, policy: P)
    where
        P: AuthorizationPolicy + 'static,
    {
        self.policies.push(Box::new(policy));
    }

    pub fn evaluate(&self, ctx: &AuthorizationContext) -> AccessDecision {
        for policy in &self.policies {
            if let Some(decision) = policy.evaluate(ctx) {
                return decision;
            }
        }

        // Default deny (secure by default)
        AccessDecision::Deny
    }
}
