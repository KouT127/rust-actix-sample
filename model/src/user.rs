use super::schema::users;
use chrono::{NaiveDateTime, Utc};

use serde::{Deserialize, Serialize};
use std::default::Default;
use validator::Validate;

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone, PartialEq)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Default for NewUser {
    fn default() -> NewUser {
        NewUser {
            name: "".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

impl NewUser {
    pub fn to_user(&self, id: u64) -> User {
        User {
            id,
            name: self.name.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
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
            id: user.id,
            nickname: user.name.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at.unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserPayload {
    #[validate(length(min = 1, max = 20))]
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
