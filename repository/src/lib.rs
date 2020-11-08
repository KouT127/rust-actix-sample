#[macro_use]
extern crate diesel;

use async_trait::async_trait;
use chrono::Utc;
use diesel::result::Error::RollbackTransaction;
use diesel::{insert_into, select, update, MysqlConnection};
use model::context::{MySqlPool, MysqlPooled, Repository};
use model::task::Task;
use model::user::User;
use mysql_async::{Params, Value};
use quaint::connector::ResultSetIterator;
use quaint::connector::SqlFamily::Mysql;
use quaint::prelude::*;
use quaint::single::Quaint;
use quaint::visitor::Visitor;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::ops::{Add, Deref};

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

#[async_trait]
pub trait TaskRepository {
    async fn find_task(task_id: u64) -> anyhow::Result<(Task, User)>;
}

#[async_trait]
impl TaskRepository for Repository {
    async fn find_task(task_id: u64) -> anyhow::Result<(Task, User)> {
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
                id: Some(0),
                name: "".to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: None,
            },
        ))
    }
}

#[async_trait]
pub trait UserRepository {
    async fn find_users() -> anyhow::Result<Vec<User>>;
    async fn find_user(user_id: u64) -> anyhow::Result<User>;
    async fn create_user<'a>(tx: &Transaction<'a>, user: &User) -> anyhow::Result<User>;
    async fn update_user(user: &User) -> anyhow::Result<u64>;
}

#[async_trait]
impl UserRepository for Repository {
    async fn find_users() -> anyhow::Result<Vec<User>> {
        let url = get_url_from_env();
        let conn = Quaint::new(url.as_str()).await?;
        let statement = Select::from_table("users").limit(10);
        let result_set: ResultSetIterator = conn.select(statement).await?.into_iter();
        let users: Vec<User> = result_set
            .map(|row| {
                let user: User = quaint::serde::from_row(row).unwrap();
                user
            })
            .collect();
        Ok(users)
    }

    async fn find_user(user_id: u64) -> anyhow::Result<User> {
        let url = get_url_from_env();
        let conn = Quaint::new(url.as_str()).await?;
        let statement = Select::from_table("users")
            .so_that("id".equals(user_id.to_string()))
            .limit(1);
        let row = conn.select(statement).await?.into_single().unwrap();
        let user: User = quaint::serde::from_row(row)?;
        Ok(user)
    }

    async fn create_user<'a>(tx: &Transaction<'a>, user: &User) -> anyhow::Result<User> {
        let statement = Insert::single_into("users")
            .value("name", user.name.as_str())
            .value("created_at", Utc::now())
            .value("updated_at", Utc::now())
            .build();
        let result_set = tx.insert(statement).await?;
        Ok(User {
            id: result_set.last_insert_id(),
            ..user.clone()
        })
    }

    async fn update_user(user: &User) -> anyhow::Result<u64> {
        let id = user.id.unwrap();
        let url = get_url_from_env();
        let conn = Quaint::new(url.as_str()).await?;
        let tx = conn.start_transaction().await?;
        let statement = Update::table("users")
            .set("name", user.name.as_str())
            .so_that("id".equals(id.to_string()));

        let affected_rows_count = tx.update(statement).await?;
        tx.commit().await?;

        Ok(affected_rows_count)
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
    use model::user::User;
    use quaint::ast::Insert;
    use quaint::prelude::*;
    use quaint::single::Quaint;

    // fn new_user(tx: &Transaction) -> User {
    //     let new = NewUser {
    //         name: "insert".to_string(),
    //         created_at: Utc::now()
    //             .duration_trunc(Duration::seconds(1))
    //             .unwrap()
    //             .naive_utc(),
    //         updated_at: Some(
    //             Utc::now()
    //                 .duration_trunc(Duration::seconds(1))
    //                 .unwrap()
    //                 .naive_utc(),
    //         ),
    //     };
    //     Repository::create_user(&new).unwrap()
    // }

    #[tokio::test]
    async fn build_query() {
        assert!(dotenv::from_filename(".env.test").is_ok());
        let url = get_url_from_env();
        let conn = Quaint::new(url.as_str()).await.unwrap();
        let tx = conn.start_transaction().await.unwrap();

        let insert = Insert::single_into("users")
            .value("name", "test")
            .value("created_at", Utc::now())
            .value("updated_at", Utc::now())
            .build();
        let result = tx.insert(insert).await;
        println!("{:?}", result);
    }

    // #[test]
    // fn test_find_users() {
    //     assert!(dotenv::from_filename(".env.test").is_ok());
    //
    //     let url = get_url_from_env();
    //     let conn = Quaint::new(&url).await.unwrap();
    //
    //     let conn = pool.get().unwrap();
    //     let inserted_user = new_user(&conn);
    //     let inserted_user2 = new_user(&conn);
    //     let expected_users = [inserted_user2, inserted_user];
    // }

    #[tokio::test]
    async fn test_create_user() {
        assert!(dotenv::from_filename(".env.test").is_ok());
        let url = get_url_from_env();
        let conn = Quaint::new(url.as_str()).await.unwrap();
        let tx = conn.start_transaction().await.unwrap();
        let created_user = Repository::create_user(
            &tx,
            &User {
                id: None,
                name: "created-1".to_owned(),
                created_at: Utc::now().naive_utc(),
                updated_at: Some(Utc::now().naive_utc()),
            },
        )
        .await
        .unwrap();

        assert!(created_user.id.is_some())
    }

    // #[test]
    // fn test_update_user() {
    //     assert!(dotenv::from_filename(".env.test").is_ok());
    //     let url = get_url_from_env();
    //     let pool = new_pool(url, 1);
    //     let conn = pool.get().unwrap();
    //
    //     let _ = conn.transaction::<(), Error, _>(|| {
    //         let inserted_user = new_user(&conn);
    //         let update_user = User {
    //             id: inserted_user.id,
    //             name: "updated".to_string(),
    //             created_at: inserted_user.created_at,
    //             updated_at: Some(
    //                 Utc::now()
    //                     .duration_trunc(Duration::seconds(1))
    //                     .unwrap()
    //                     .naive_utc(),
    //             ),
    //         };
    //
    //         let updated_user = Repository::update_user(&conn, &update_user).unwrap();
    //         let user = Repository::find_user(&conn, updated_user.id);
    //         assert!(user.is_ok());
    //         assert_eq!(updated_user, user.unwrap());
    //         Err(RollbackTransaction)
    //     });
    // }
}
