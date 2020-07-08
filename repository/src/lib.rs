#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error::RollbackTransaction;
use diesel::{insert_into, select, update, MysqlConnection};
use model::context::MySqlPool;
use model::user::{NewUser, User};

no_arg_sql_function!(
    last_insert_id,
    diesel::types::Unsigned<diesel::types::Bigint>
);

pub async fn new_pool() -> MySqlPool {
    let url = std::env::var("DATABASE_URL").expect("Database URL is not exists");
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .map_err(|error| panic!(error))
        .unwrap()
}

pub fn find_users(conn: &MysqlConnection) -> anyhow::Result<Vec<User>> {
    use model::schema::users::dsl::{id, users};
    users
        .limit(10)
        .order(id.desc())
        .load::<User>(conn)
        .map_err(|error| anyhow::Error::new(error))
}

pub fn create_user<'a>(conn: &MysqlConnection, user: &NewUser<'a>) -> anyhow::Result<User> {
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

pub fn update_user(conn: &MysqlConnection, user: &User) -> anyhow::Result<()> {
    use model::schema::users::dsl::{id, name, users};
    update(users.filter(id.eq(user.id)))
        .set(name.eq("test"))
        .execute(conn)?;
    Ok(())
}
