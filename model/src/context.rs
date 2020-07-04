use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use r2d2::{Pool, PooledConnection};
use tera::Tera;

pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooled = PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct Context {
    pub pool: MySqlPool,
    pub template: Tera,
}
