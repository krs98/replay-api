use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use validator::validate_email;

use crate::modules::error::AppError;

#[derive(sqlx::FromRow)]
pub struct User {
    pub username: Username,
    pub email: Email,
    pub password: Password,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub struct Username(String);

impl Username {
    pub fn into_inner(self) -> String {
        self.0
    }
}

pub const MAX_USERNAME_LENGTH: usize = 30;

impl TryFrom<String> for Username {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(AppError::UsernameIsEmpty)
        }        

        if value.len() > MAX_USERNAME_LENGTH {
            return Err(AppError::UsernameTooLong)
        }

        Ok(Username(value))
    }
}

#[derive(Debug, Clone, sqlx::Type)]
pub struct Email(String);

impl Email {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Email {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !validate_email(&value) {
            return Err(AppError::InvalidEmail)
        }

        Ok(Email(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
pub struct Password(String);

impl Password {
    pub fn into_inner(self) -> String {
        self.0
    }
}

pub const MIN_PASSWORD_LENGTH: usize = 6;
pub const MAX_PASSWORD_LENGTH: usize = 20;

impl TryFrom<String> for Password {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < MIN_PASSWORD_LENGTH {
            return Err(AppError::PasswordTooShort)
        }

        if value.len() > MAX_PASSWORD_LENGTH {
            return Err(AppError::PasswordTooLong)
        }

        Ok(Password(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::{error::AppError, users::model::Password};

    #[test]
    fn test_short_password() {
        let password = String::from("short");
        let password: Result<Password, AppError> = password.try_into();
        assert_eq!(password, Err(AppError::PasswordTooShort));
    }

    #[test]
    fn test_long_password() {
        let password = String::from("thispasswordiswaytoolong");
        let password: Result<Password, AppError> = password.try_into();
        assert_eq!(password, Err(AppError::PasswordTooLong));
    }

    #[test]
    fn test_valid_password() {
        let password = String::from("valid_password");
        assert_eq!(password.clone().try_into(), Ok(Password(password)));
    }
}
