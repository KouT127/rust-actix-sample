#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error::RollbackTransaction;
use diesel::{insert_into, select, update, MysqlConnection};
use model::context::{MySqlPool, MysqlPooled, Repository};
use model::task::Task;
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

pub trait TaskRepository {
    fn find_task(conn: &MysqlPooled, task_id: u64) -> anyhow::Result<(Task, User)>;
}

impl TaskRepository for Repository {
    fn find_task(conn: &MysqlPooled, task_id: u64) -> anyhow::Result<(Task, User)> {
        use model::schema::*;

        tasks::table
            .inner_join(users::table.on(users::dsl::id.eq(tasks::dsl::id)))
            .filter(tasks::dsl::id.eq(task_id))
            .first::<(Task, User)>(conn)
            .map_err(anyhow::Error::new)
    }
}

pub trait UserRepository {
    fn find_users(conn: &MysqlPooled) -> anyhow::Result<Vec<User>>;
    fn find_user(conn: &MysqlPooled, user_id: u64) -> anyhow::Result<User>;
    fn create_user(conn: &MysqlPooled, user: &NewUser) -> anyhow::Result<User>;
    fn update_user(conn: &MysqlPooled, user: &User) -> anyhow::Result<User>;
}

impl UserRepository for Repository {
    fn find_users(conn: &MysqlPooled) -> anyhow::Result<Vec<User>> {
        use model::schema::users::dsl::{id, users};
        users
            .limit(10)
            .order(id.desc())
            .load::<User>(conn)
            .map_err(anyhow::Error::new)
    }

    fn find_user(conn: &MysqlPooled, user_id: u64) -> anyhow::Result<User> {
        use model::schema::users::dsl::{id, users};
        users
            .filter(id.eq(user_id))
            .first::<User>(conn)
            .map_err(anyhow::Error::new)
    }

    fn create_user(conn: &MysqlPooled, user: &NewUser) -> anyhow::Result<User> {
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
    use chrono::{Duration, DurationRound, Utc};
    use diesel::result::Error;
    use diesel::result::Error::RollbackTransaction;
    use diesel::Connection;
    use model::context::{MysqlPooled, Repository};
    use model::user::{NewUser, User};

    fn new_user(conn: &MysqlPooled) -> User {
        let new = NewUser {
            name: "insert".to_string(),
            created_at: Utc::now()
                .duration_trunc(Duration::seconds(1))
                .unwrap()
                .naive_utc(),
            updated_at: Some(
                Utc::now()
                    .duration_trunc(Duration::seconds(1))
                    .unwrap()
                    .naive_utc(),
            ),
        };
        Repository::create_user(&conn, &new).unwrap()
    }

    #[test]
    fn test_find_users() {
        assert!(dotenv::from_filename(".env.test").is_ok());
        let url = get_url_from_env();
        let pool = new_pool(url, 1);
        let conn = pool.get().unwrap();
        let _ = conn.transaction::<(), Error, _>(|| {
            let inserted_user = new_user(&conn);
            let inserted_user2 = new_user(&conn);
            let expected_users = [inserted_user2, inserted_user];

            let users = Repository::find_users(&conn);
            assert!(users.is_ok());
            assert_eq!(users.unwrap(), expected_users);
            Err(RollbackTransaction)
        });
    }

    #[test]
    fn test_create_user() {
        assert!(dotenv::from_filename(".env.test").is_ok());
        let url = get_url_from_env();
        let pool = new_pool(url, 1);
        let conn = pool.get().unwrap();

        let _ = conn.transaction::<(), Error, _>(|| {
            let inserted_user = new_user(&conn);
            let user = Repository::find_user(&conn, inserted_user.id);
            assert!(user.is_ok());
            assert_eq!(inserted_user, user.unwrap());
            Err(RollbackTransaction)
        });
    }

    #[test]
    fn test_update_user() {
        assert!(dotenv::from_filename(".env.test").is_ok());
        let url = get_url_from_env();
        let pool = new_pool(url, 1);
        let conn = pool.get().unwrap();

        let _ = conn.transaction::<(), Error, _>(|| {
            let inserted_user = new_user(&conn);
            let update_user = User {
                id: inserted_user.id,
                name: "updated".to_string(),
                created_at: inserted_user.created_at,
                updated_at: Some(
                    Utc::now()
                        .duration_trunc(Duration::seconds(1))
                        .unwrap()
                        .naive_utc(),
                ),
            };

            let updated_user = Repository::update_user(&conn, &update_user).unwrap();
            let user = Repository::find_user(&conn, updated_user.id);
            assert!(user.is_ok());
            assert_eq!(updated_user, user.unwrap());
            Err(RollbackTransaction)
        });
    }
}
