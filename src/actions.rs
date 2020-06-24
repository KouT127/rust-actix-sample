use super::diesel::prelude::*;
use crate::models::User;
use crate::{models, MysqlPooled};

pub fn find_users(pool: MysqlPooled) -> Result<Vec<models::User>, diesel::result::Error> {
    use super::schema::users::dsl::*;
    users.limit(5).load::<User>(&*pool)
}
