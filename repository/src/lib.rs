#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error::RollbackTransaction;
use diesel::{insert_into, select, update, MysqlConnection};
use model::context::{MySqlPool, Repository};
use model::user::{NewUser, User};

no_arg_sql_function!(
    last_insert_id,
    diesel::types::Unsigned<diesel::types::Bigint>
);

pub async fn new_pool() -> MySqlPool {
    let url = std::env::var("DATABASE_URL").expect("Database URL is not exists");
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(2)
        .build(manager)
        .expect("Failed to connect")
}

pub fn establish_connection() -> ConnectionResult<MysqlConnection> {
    dotenv::from_filename(".env.test").ok().unwrap_or_default();
    let url = std::env::var("DATABASE_URL").expect("Database URL is not exists");
    diesel::MysqlConnection::establish(url.as_str())
}

pub trait UserRepository {
    fn find_users(&self) -> anyhow::Result<Vec<User>>;
    fn create_user<'a>(&self, user: &NewUser<'a>) -> anyhow::Result<User>;
    fn update_user(&self, user: &User) -> anyhow::Result<User>;
}

impl UserRepository for Repository {
    fn find_users(&self) -> anyhow::Result<Vec<User>> {
        use model::schema::users::dsl::{id, users};
        users
            .limit(10)
            .order(id.desc())
            .load::<User>(&self.conn)
            .map_err(|error| anyhow::Error::new(error))
    }

    fn create_user<'a>(&self, user: &NewUser<'a>) -> anyhow::Result<User> {
        use model::schema::users::dsl::users;
        insert_into(users)
            .values(user)
            .execute(&self.conn)
            .map_err(|error| {
                println!("{}", error);
                RollbackTransaction
            })?;
        let generated_id: u64 = select(last_insert_id).first(&self.conn).unwrap();
        let new_user = user.to_user(generated_id);
        Ok(new_user)
    }

    fn update_user(&self, user: &User) -> anyhow::Result<User> {
        use model::schema::users::dsl::{id, name, users};
        update(users.filter(id.eq(user.id)))
            .set(name.eq(user.name.to_owned()))
            .execute(&self.conn)
            .map_err(|error| {
                println!("{}", error);
                RollbackTransaction
            })?;
        Ok(user.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::establish_connection;

    #[test]
    fn establishing_connection() {
        let result = establish_connection();
        assert!(result.is_ok())
    }
}
