use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct DateTime(i64);

impl DateTime {
    pub fn new(datetime: i64) -> Self {
        Self(datetime)
    }

    pub fn datetime(&self) -> i64 {
        self.0
    }
}

impl Deref for DateTime {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
