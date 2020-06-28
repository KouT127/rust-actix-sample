use model::user::{NewUser, User};
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};

pub async fn new_pool() -> MySqlPool {
    let url = std::env::var("DATABASE_URL").expect("Database URL is not exists");

    MySqlPool::builder()
        .min_size(0)
        .max_size(5)
        .build(&url)
        .await
        .expect("Failed to mysql")
}

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
        .await?;
    Ok(users)
}

pub async fn create_user(conn: &MySqlPool, user: &mut NewUser) -> anyhow::Result<u64> {
    sqlx::query("INSERT INTO users (name, created_at, updated_at) value (?, ? ,?)")
        .bind(user.name.to_string())
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(conn)
        .await?;

    let insert_id = fetch_last_insert_id(conn).await?;
    user.id = Some(insert_id);
    Ok(insert_id)
}

pub async fn update_user(conn: &MySqlPool, user: &User) -> anyhow::Result<()> {
    sqlx::query("UPDATE users SET name = ?,  updated_at = ? WHERE id = ?")
        .bind(user.name.to_owned())
        .bind(user.updated_at)
        .bind(user.id)
        .execute(conn)
        .await?;
    Ok(())
}

async fn fetch_last_insert_id(conn: &MySqlPool) -> anyhow::Result<u64> {
    let insert_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .map(|row: MySqlRow| u64::from(row.get::<u64, _>(0)))
        .fetch_one(conn)
        .await?;

    Ok(insert_id)
}
