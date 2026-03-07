use domain::tenant::environment::Environment;

use crate::SubjectContex;

use super::{
    action::AuthorizationAction,
    attributes::AuthorizationAttributes,
    policys::{ApplicationAuthorizationPolicy, ApplicationStoredPolicy},
    relation::AuthorizationRelations,
    resource_type::AuthorizationResourceType,
    AccessDecision,
};

pub struct AuthorizationContext<'a> {
    pub subject: &'a SubjectContex,
    pub action: AuthorizationAction,

    pub resource_type: AuthorizationResourceType,

    pub subject_attributes: AuthorizationAttributes,
    pub resource_attributes: AuthorizationAttributes,

    pub relations: AuthorizationRelations,
    pub environment: Environment,
}

pub struct AuthorizationEngine {
    guard_policies: Vec<ApplicationStoredPolicy>,
    dynamic_policies: Vec<Box<dyn ApplicationAuthorizationPolicy>>,
}

impl AuthorizationEngine {
    pub fn new() -> Self {
        Self {
            dynamic_policies: Vec::new(),
            guard_policies: Vec::new(),
        }
    }

    pub fn register_policy<P>(&mut self, policy: P)
    where
        P: ApplicationAuthorizationPolicy + 'static,
    {
        self.dynamic_policies.push(Box::new(policy));
    }

    pub fn evaluate(&self, ctx: &AuthorizationContext) -> AccessDecision {
        // 1. Guard policies (hard security rules)
        for policy in &self.guard_policies {
            if let Some(decision) = policy.evaluate(ctx) {
                return decision;
            }
        }

        // 2. Dynamic policies (editable)
        for policy in &self.dynamic_policies {
            if let Some(decision) = policy.evaluate(ctx) {
                return decision;
            }
        }

        // 3. Secure default
        AccessDecision::Deny
    }
}
