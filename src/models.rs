use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct NewUser {
    pub id: Option<i64>,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserResponse {
    pub fn from_user(user: &User) -> Self {
        UserResponse {
            id: user.id,
            name: user.name.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
    pub fn from_new_user(user: &NewUser) -> Self {
        UserResponse {
            id: user.id.unwrap(),
            name: user.name.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
