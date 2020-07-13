#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error::RollbackTransaction;
use diesel::{insert_into, select, update, MysqlConnection};
use model::context::{MySqlPool, MysqlPooled, Repository};
use model::user::{NewUser, User};

no_arg_sql_function!(
    last_insert_id,
    diesel::types::Unsigned<diesel::types::Bigint>
);

pub fn get_url_from_env() -> String {
    std::env::var("DATABASE_URL").expect("Database URL is not exists")
}

pub fn new_pool(url: String, pool_size: u32) -> MySqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .expect("Failed to connect")
}

pub trait UserRepository {
    fn find_users(conn: &MysqlPooled) -> anyhow::Result<Vec<User>>;
    fn create_user<'a>(conn: &MysqlPooled, user: &NewUser<'a>) -> anyhow::Result<User>;
    fn update_user(conn: &MysqlPooled, user: &User) -> anyhow::Result<User>;
}

impl UserRepository for Repository {
    fn find_users(conn: &MysqlPooled) -> anyhow::Result<Vec<User>> {
        use model::schema::users::dsl::{id, users};
        users
            .limit(10)
            .order(id.desc())
            .load::<User>(conn)
            .map_err(|error| anyhow::Error::new(error))
    }

    fn create_user<'a>(conn: &MysqlPooled, user: &NewUser<'a>) -> anyhow::Result<User> {
        use model::schema::users::dsl::users;
        insert_into(users)
            .values(user)
            .execute(conn)
            .map_err(|error| {
                println!("{}", error);
                RollbackTransaction
            })?;
        let generated_id: u64 = select(last_insert_id).first(conn).unwrap();
        let new_user = user.to_user(generated_id);
        Ok(new_user)
    }

    fn update_user(conn: &MysqlPooled, user: &User) -> anyhow::Result<User> {
        use model::schema::users::dsl::{id, name, users};
        update(users.filter(id.eq(user.id)))
            .set(name.eq(user.name.to_owned()))
            .execute(conn)
            .map_err(|error| {
                println!("{}", error);
                RollbackTransaction
            })?;
        Ok(user.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_url_from_env, new_pool, UserRepository};
    use model::context::Repository;
    use model::user::User;

    #[test]
    fn finding_user() {
        assert!(dotenv::from_filename(".env.test").is_ok());
        let url = get_url_from_env();
        let pool = new_pool(url, 1);
        let conn = pool.get();
        assert!(conn.is_ok());
        let users = Repository::find_users(&conn.unwrap());
        assert!(users.is_ok());
        let expected = Vec::<User>::new();
        assert_eq!(users.unwrap(), expected);
    }
}
