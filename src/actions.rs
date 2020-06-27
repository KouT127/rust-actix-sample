use crate::models::{NewUser, User};
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};
use std::ops::Deref;

pub async fn find_users(conn: &MySqlPool) -> anyhow::Result<Vec<User>> {
    // ex: Use macro
    // let users = sqlx::query_as!(User, "SELECT * FROM users limit 10")
    //     .fetch_all(conn)
    //     .await;

    let users = sqlx::query("SELECT * FROM users limit 10")
        .map(|row: MySqlRow| User {
            id: row.get("id"),
            name: row.get("name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_all(conn)
        .await;

    match users {
        Ok(users) => Ok(users),
        _ => Err(anyhow::Error::msg("error")),
    }
}

pub async fn create_user(conn: &MySqlPool, user: &mut NewUser) -> anyhow::Result<u64> {
    let mut tx = conn.begin().await?;
    let affected = sqlx::query("INSERT INTO users (name, created_at, updated_at) value (?, ? ,?)")
        .bind(user.name.to_string())
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(conn)
        .await;

    if let Err(affected) = affected {
        tx.rollback().await?;
        return Err(anyhow::Error::new(affected));
    }

    let insert_id = fetch_last_insert_id(conn).await;
    if let Err(res) = insert_id {
        tx.rollback().await?;
        return Err(res);
    }

    tx.commit().await?;
    let id = insert_id?;
    user.id = Some(id);
    Ok(id)
}

async fn fetch_last_insert_id(conn: &MySqlPool) -> anyhow::Result<u64> {
    let insert_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .map(|row: MySqlRow| u64::from(row.get::<u64, _>(0)))
        .fetch_one(conn)
        .await?;

    Ok(insert_id)
}
