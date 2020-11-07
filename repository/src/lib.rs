#[macro_use]
extern crate diesel;

use chrono::Utc;
use diesel::result::Error::RollbackTransaction;
use diesel::{insert_into, select, update, MysqlConnection};
use model::context::{MySqlPool, MysqlPooled, Repository};
use model::task::Task;
use model::user::{NewUser, User};
use mysql_async::prelude::Queryable;
use mysql_async::{Params, Value};
use quaint::connector::ResultSetIterator;
use quaint::connector::SqlFamily::Mysql;
use quaint::prelude::Select;
use quaint::visitor::Visitor;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::ops::{Add, Deref};

no_arg_sql_function!(
    last_insert_id,
    diesel::types::Unsigned<diesel::types::Bigint>
);

pub fn get_url_from_env() -> String {
    std::env::var("DATABASE_URL").expect("Database URL is not exists")
}

pub fn new_pool(url: String, pool_size: u32) -> MySqlPool {
    use diesel::r2d2::ConnectionManager;
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
        Ok((
            Task {
                id: 0,
                user_id: 0,
                title: "".to_string(),
                is_done: false,
                created_at: Utc::now().naive_utc(),
                updated_at: None,
            },
            User {
                id: 0,
                name: "".to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: None,
            },
        ))
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
        Ok(Vec::new())
    }

    fn find_user(conn: &MysqlPooled, user_id: u64) -> anyhow::Result<User> {
        Ok(User {
            id: 0,
            name: "".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        })
    }

    fn create_user(conn: &MysqlPooled, user: &NewUser) -> anyhow::Result<User> {
        Ok(User {
            id: 0,
            name: "".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        })
    }

    fn update_user(conn: &MysqlPooled, user: &User) -> anyhow::Result<User> {
        Ok(User {
            id: 0,
            name: "".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        })
    }
}

async fn build_query() -> Vec<User> {
    use quaint::{prelude::*, single::Quaint};
    let conn = Quaint::new("").await;

    // let conditions = "word"
    //     .equals("meow")
    //     .and("age".less_than(10))
    //     .and("paw".equals("warm"));

    let query = Select::from_table("users").limit(10);
    let result = conn.unwrap().select(query).await.unwrap();
    let result_iterator: ResultSetIterator = result.into_iter();
    result_iterator
        .map(|row| User {
            id: row[0].as_i64().unwrap() as u64,
            name: row[1].as_str().unwrap().to_owned(),
            created_at: Utc::now().naive_local(),
            updated_at: None,
        })
        .collect::<Vec<User>>()
}

async fn hoge() {
    use mysql_async::prelude::*;

    let pool = mysql_async::Pool::new("");
    let mut conn = pool.get_conn().await.unwrap();
    let mut v = HashMap::with_hasher(BuildHasherDefault::default());

    let statement = "SELECT customer_id, amount, account_name 
        from users 
        where id = :foo"
        .to_owned();

    let statement = statement.add("o = :ok");
    v.insert("test".to_owned(), Value::Bytes("".to_owned().into_bytes()));

    let statement = conn.prep(statement).await.unwrap();

    let users = conn
        .exec_map(
            &statement,
            Params::Named(v),
            |(id, name, created_at, updated_at)| User {
                id,
                name,
                created_at,
                updated_at,
            },
        )
        .await;
    let users = conn
        .query_map(
            "SELECT customer_id, amount, account_name from payment",
            |(id, name, created_at, updated_at)| User {
                id,
                name,
                created_at,
                updated_at,
            },
        )
        .await;
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
    use quaint::ast::Insert;
    use quaint::prelude::*;
    use quaint::single::Quaint;

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

    #[tokio::test]
    async fn build_query() {
        tokio::spawn(async {
            assert!(dotenv::from_filename(".env.test").is_ok());
            let url = get_url_from_env();
            let conn = Quaint::new(url.as_str()).await.unwrap();
            let insert = Insert::single_into("users").value("name", "test").build();
            match conn.insert(insert.clone()).await {
                Err(error) => println!("{:?}", error),
                _ => {}
            }
            let result = conn.insert(insert).await;
            println!("{:?}", result);
        })
        .await
        .unwrap();
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
