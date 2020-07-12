use diesel::r2d2::ConnectionManager;
use diesel::{Connection, MysqlConnection};
use r2d2::{Pool, PooledConnection};
use tera::Tera;

pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooled = PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct Context {
    pub pool: MySqlPool,
    pub template: Tera,
}

pub struct Handler;

pub struct Repository {
    pub conn: MysqlPooled,
}

impl Repository {
    pub fn build(pool: &MySqlPool) -> anyhow::Result<Self> {
        let conn = pool.get()?;
        Ok(Self { conn })
    }
}
