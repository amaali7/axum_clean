use std::{fmt, str::FromStr};

use domain::{value_objects::password::NoneHashedPassword, HashedPassword, Password};
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfrastructurePassword {
    Hashed(InfrastructureHashedPassword),
    NoneHashed(InfrastructureNoneHashedPassword),
}

impl fmt::Display for InfrastructurePassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfrastructurePassword::Hashed(hashed_password) => write!(f, "{}", hashed_password),
            InfrastructurePassword::NoneHashed(none_hashed_password) => {
                write!(f, "{}", none_hashed_password)
            }
        }
    }
}

impl Default for InfrastructurePassword {
    fn default() -> Self {
        InfrastructurePassword::NoneHashed(InfrastructureNoneHashedPassword::default())
    }
}

impl TryFrom<Password> for InfrastructurePassword {
    fn try_from(value: Password) -> InfrastructureResult<Self> {
        match value {
            Password::Hashed(hashed_password) => Ok(Self::Hashed(
                InfrastructureHashedPassword::try_from(hashed_password)?,
            )),
            Password::NoneHashed(none_hashed_password) => Ok(Self::NoneHashed(
                InfrastructureNoneHashedPassword::try_from(none_hashed_password)?,
            )),
        }
    }

    type Error = InfrastructureError;
}

impl TryFrom<InfrastructurePassword> for Password {
    fn try_from(value: InfrastructurePassword) -> InfrastructureResult<Self> {
        match value {
            InfrastructurePassword::Hashed(hashed_password) => {
                Ok(Self::Hashed(HashedPassword::try_from(hashed_password)?))
            }
            InfrastructurePassword::NoneHashed(none_hashed_password) => Ok(Self::NoneHashed(
                NoneHashedPassword::try_from(none_hashed_password)?,
            )),
        }
    }
    type Error = InfrastructureError;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InfrastructureNoneHashedPassword(String);

impl fmt::Display for InfrastructureNoneHashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl InfrastructureNoneHashedPassword {
    pub fn new(password: &str) -> InfrastructureResult<Self> {
        if password.len() < 8 {
            return Err(InfrastructureError::ValidationError(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        if password.len() > 100 {
            return Err(InfrastructureError::ValidationError(
                "Password too long".to_string(),
            ));
        }

        // Check for common passwords in real implementation
        Ok(Self(password.to_string()))
    }

    pub fn none_hashed_password(&self) -> String {
        self.0.clone()
    }
}

impl FromStr for InfrastructureNoneHashedPassword {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<NoneHashedPassword> for InfrastructureNoneHashedPassword {
    fn try_from(value: NoneHashedPassword) -> InfrastructureResult<Self> {
        Self::new(&value.none_hashed_password())
    }

    type Error = InfrastructureError;
}

impl TryFrom<InfrastructureNoneHashedPassword> for NoneHashedPassword {
    fn try_from(value: InfrastructureNoneHashedPassword) -> InfrastructureResult<Self> {
        Self::new(&value.none_hashed_password()).map_err(|err| InfrastructureError::Domain(err))
    }

    type Error = InfrastructureError;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InfrastructureHashedPassword(String);

impl fmt::Display for InfrastructureHashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl InfrastructureHashedPassword {
    pub fn new(hashed_password: &str) -> InfrastructureResult<Self> {
        if hashed_password.len() < 43 || hashed_password.len() > 128 {
            return Err(InfrastructureError::ValidationError(
                "Its not Hashed Password".to_string(),
            ));
        }

        // Check for common passwords in real implementation
        Ok(Self(hashed_password.to_string()))
    }

    pub fn hashed_password(&self) -> String {
        self.0.clone()
    }
}

impl FromStr for InfrastructureHashedPassword {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<HashedPassword> for InfrastructureHashedPassword {
    fn try_from(value: HashedPassword) -> InfrastructureResult<Self> {
        Self::new(&value.hashed_password())
    }

    type Error = InfrastructureError;
}

impl TryFrom<InfrastructureHashedPassword> for HashedPassword {
    fn try_from(value: InfrastructureHashedPassword) -> InfrastructureResult<Self> {
        Self::new(&value.hashed_password()).map_err(|err| InfrastructureError::Domain(err))
    }
    type Error = InfrastructureError;
}
