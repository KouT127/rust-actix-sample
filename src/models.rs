use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// .load::<User>()ができない場合、
// Databaseのカラムの型、構造体の型が対応していない可能性がある。
#[derive(Queryable)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub update_at: Option<NaiveDateTime>,
}

impl UserResponse {
    pub(crate) fn from(user: &User) -> UserResponse {
        UserResponse {
            id: user.id,
            name: user.name.clone(),
            created_at: user.created_at,
            update_at: user.update_at,
        }
    }
}
