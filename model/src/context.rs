use sqlx::MySqlPool;
use tera::Tera;

pub struct Context {
    pub pool: MySqlPool,
    pub template: Tera,
}
