use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// .load::<User>()ができない場合、
// Databaseのカラムの型、構造体の型が対応していない可能性がある。
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Option<u32>,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NewUser {
    pub id: Option<u32>,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserResponse {
    pub(crate) fn from(user: &User) -> UserResponse {
        UserResponse {
            id: user.id.unwrap(),
            name: user.name.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
