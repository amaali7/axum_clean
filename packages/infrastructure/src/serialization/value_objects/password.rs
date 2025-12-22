use std::{fmt, str::FromStr};

use domain::{value_objects::password::NoneHashedPassword, HashedPassword, Password};
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedPassword {
    Hashed(SerializedHashedPassword),
    NoneHashed(SerializedNoneHashedPassword),
}

impl fmt::Display for SerializedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializedPassword::Hashed(hashed_password) => write!(f, "{}", hashed_password),
            SerializedPassword::NoneHashed(none_hashed_password) => {
                write!(f, "{}", none_hashed_password)
            }
        }
    }
}

impl Default for SerializedPassword {
    fn default() -> Self {
        SerializedPassword::NoneHashed(SerializedNoneHashedPassword::default())
    }
}

impl TryFrom<Password> for SerializedPassword {
    fn try_from(value: Password) -> InfrastructureResult<Self> {
        match value {
            Password::Hashed(hashed_password) => Ok(Self::Hashed(
                SerializedHashedPassword::try_from(hashed_password)?,
            )),
            Password::NoneHashed(none_hashed_password) => Ok(Self::NoneHashed(
                SerializedNoneHashedPassword::try_from(none_hashed_password)?,
            )),
        }
    }

    type Error = InfrastructureError;
}

impl TryFrom<SerializedPassword> for Password {
    fn try_from(value: SerializedPassword) -> InfrastructureResult<Self> {
        match value {
            SerializedPassword::Hashed(hashed_password) => {
                Ok(Self::Hashed(HashedPassword::try_from(hashed_password)?))
            }
            SerializedPassword::NoneHashed(none_hashed_password) => Ok(Self::NoneHashed(
                NoneHashedPassword::try_from(none_hashed_password)?,
            )),
        }
    }
    type Error = InfrastructureError;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SerializedNoneHashedPassword(String);

impl fmt::Display for SerializedNoneHashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SerializedNoneHashedPassword {
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

impl FromStr for SerializedNoneHashedPassword {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<NoneHashedPassword> for SerializedNoneHashedPassword {
    fn try_from(value: NoneHashedPassword) -> InfrastructureResult<Self> {
        Self::new(&value.none_hashed_password())
    }

    type Error = InfrastructureError;
}

impl TryFrom<SerializedNoneHashedPassword> for NoneHashedPassword {
    fn try_from(value: SerializedNoneHashedPassword) -> InfrastructureResult<Self> {
        Self::new(&value.none_hashed_password()).map_err(|err| InfrastructureError::Domain(err))
    }

    type Error = InfrastructureError;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SerializedHashedPassword(String);

impl fmt::Display for SerializedHashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SerializedHashedPassword {
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

impl FromStr for SerializedHashedPassword {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<HashedPassword> for SerializedHashedPassword {
    fn try_from(value: HashedPassword) -> InfrastructureResult<Self> {
        Self::new(&value.hashed_password())
    }

    type Error = InfrastructureError;
}

impl TryFrom<SerializedHashedPassword> for HashedPassword {
    fn try_from(value: SerializedHashedPassword) -> InfrastructureResult<Self> {
        Self::new(&value.hashed_password()).map_err(|err| InfrastructureError::Domain(err))
    }
    type Error = InfrastructureError;
}
