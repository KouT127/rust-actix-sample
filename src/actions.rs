use crate::models::{NewUser, User};
use chrono::Utc;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};

pub async fn find_users(conn: &MySqlPool) -> anyhow::Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users limit 10")
        .fetch_all(conn)
        .await;

    match users {
        Ok(users) => Ok(users),
        _ => Err(anyhow::Error::msg("error")),
    }
}

pub async fn find_users2(conn: &MySqlPool) -> anyhow::Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users limit 10")
        .fetch_all(conn)
        .await;

    match users {
        Ok(users) => Ok(users),
        _ => Err(anyhow::Error::msg("error")),
    }
}

pub async fn create_user(conn: &MySqlPool) -> anyhow::Result<u64> {
    let mut tx = conn.begin().await?;
    let user = NewUser {
        id: None,
        name: "test".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let affected = sqlx::query("INSERT INTO users (name, created_at, updated_at) value (?, ? ,?)")
        .bind(user.name)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(conn)
        .await;

    if let Err(affected) = affected {
        tx.rollback().await?;
        return Err(anyhow::Error::new(affected));
    }

    let insert_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .map(|row: MySqlRow| row.try_get::<u64, _>(0))
        .fetch_one(&mut tx)
        .await;

    if let Err(res) = insert_id {
        tx.rollback().await?;
        return Err(anyhow::Error::new(res));
    }

    tx.commit().await?;
    match insert_id {
        Ok(insert_id) => Ok(insert_id.unwrap()),
        _ => Err(anyhow::Error::new(insert_id.err().unwrap())),
    }
}
