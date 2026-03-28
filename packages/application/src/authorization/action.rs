use std::ops::DerefMut;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AuthorizationAction(String);

impl AuthorizationAction {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
    pub fn action(&self) -> String {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for AuthorizationAction {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AuthorizationAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for AuthorizationAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
