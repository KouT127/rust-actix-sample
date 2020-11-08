use super::schema::users;
use chrono::{NaiveDateTime, Utc};

use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Queryable, Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub id: Option<u64>,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindUsersResponse {
    pub responses: Vec<UserResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u64,
    pub nickname: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserResponse {
    pub fn from_user(user: &User) -> Self {
        UserResponse {
            id: user.id.unwrap(),
            nickname: user.name.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at.unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPayload {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::user::UserPayload;
    use crate::validator::{Validate, ValidationErrors};

    #[test]
    fn validate_user_payload_with_valid_value() {
        let payload = UserPayload {
            name: "1".to_string(),
        };

        let result: Result<(), ValidationErrors> = payload.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn validate_user_payload_with_too_long_value() {
        let payload = UserPayload {
            name: "123456789012345678901".to_string(),
        };

        let result: Result<(), ValidationErrors> = payload.validate();
        assert!(result.is_err());
    }

    #[test]
    fn validate_user_payload_with_empty_string() {
        let payload = UserPayload {
            name: "".to_string(),
        };

        let result: Result<(), ValidationErrors> = payload.validate();
        assert!(result.is_err());
    }
}
