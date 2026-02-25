use std::{fmt, str::FromStr};

use domain::{value_objects::password::NoneHashedPassword, HashedPassword, Password};
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterfacePassword {
    Hashed(InterfaceHashedPassword),
    NoneHashed(InterfaceNoneHashedPassword),
}

impl fmt::Display for InterfacePassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfacePassword::Hashed(hashed_password) => write!(f, "{}", hashed_password),
            InterfacePassword::NoneHashed(none_hashed_password) => {
                write!(f, "{}", none_hashed_password)
            }
        }
    }
}

impl Default for InterfacePassword {
    fn default() -> Self {
        InterfacePassword::NoneHashed(InterfaceNoneHashedPassword::default())
    }
}

impl TryFrom<Password> for InterfacePassword {
    fn try_from(value: Password) -> InterfaceResult<Self> {
        match value {
            Password::Hashed(hashed_password) => Ok(Self::Hashed(
                InterfaceHashedPassword::try_from(hashed_password)?,
            )),
            Password::NoneHashed(none_hashed_password) => Ok(Self::NoneHashed(
                InterfaceNoneHashedPassword::try_from(none_hashed_password)?,
            )),
        }
    }

    type Error = InterfaceError;
}

impl TryFrom<InterfacePassword> for Password {
    fn try_from(value: InterfacePassword) -> InterfaceResult<Self> {
        match value {
            InterfacePassword::Hashed(hashed_password) => {
                Ok(Self::Hashed(HashedPassword::try_from(hashed_password)?))
            }
            InterfacePassword::NoneHashed(none_hashed_password) => Ok(Self::NoneHashed(
                NoneHashedPassword::try_from(none_hashed_password)?,
            )),
        }
    }
    type Error = InterfaceError;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InterfaceNoneHashedPassword(String);

impl fmt::Display for InterfaceNoneHashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl InterfaceNoneHashedPassword {
    pub fn new(password: &str) -> InterfaceResult<Self> {
        if password.len() < 8 {
            return Err(InterfaceError::ValidationError(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        if password.len() > 100 {
            return Err(InterfaceError::ValidationError(
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

impl FromStr for InterfaceNoneHashedPassword {
    type Err = InterfaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<NoneHashedPassword> for InterfaceNoneHashedPassword {
    fn try_from(value: NoneHashedPassword) -> InterfaceResult<Self> {
        Self::new(&value.none_hashed_password())
    }

    type Error = InterfaceError;
}

impl TryFrom<InterfaceNoneHashedPassword> for NoneHashedPassword {
    fn try_from(value: InterfaceNoneHashedPassword) -> InterfaceResult<Self> {
        Self::new(&value.none_hashed_password()).map_err(|err| InterfaceError::Domain(err))
    }

    type Error = InterfaceError;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InterfaceHashedPassword(String);

impl fmt::Display for InterfaceHashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl InterfaceHashedPassword {
    pub fn new(hashed_password: &str) -> InterfaceResult<Self> {
        if hashed_password.len() < 43 || hashed_password.len() > 128 {
            return Err(InterfaceError::ValidationError(
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

impl FromStr for InterfaceHashedPassword {
    type Err = InterfaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<HashedPassword> for InterfaceHashedPassword {
    fn try_from(value: HashedPassword) -> InterfaceResult<Self> {
        Self::new(&value.hashed_password())
    }

    type Error = InterfaceError;
}

impl TryFrom<InterfaceHashedPassword> for HashedPassword {
    fn try_from(value: InterfaceHashedPassword) -> InterfaceResult<Self> {
        Self::new(&value.hashed_password()).map_err(|err| InterfaceError::Domain(err))
    }
    type Error = InterfaceError;
}
