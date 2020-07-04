use super::schema::users;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl<'a> NewUser<'a> {
    pub fn to_user(&self, id: u64) -> User {
        User {
            id,
            name: self.name.to_owned(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl<'a> Copy for NewUser<'a> {}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
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
            updated_at: user.updated_at.unwrap(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    pub name: String,
}
