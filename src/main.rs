#[macro_use]
extern crate diesel;

pub mod actions;
pub mod models;
pub mod schema;

use crate::actions::find_users;
use crate::models::UserResponse;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::{Connection, MysqlConnection};
use r2d2::Pool;
use std::env;

type MySqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
type MysqlPooled = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct Context {
    pool: MySqlPool,
}

pub fn new_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let database_url = "mysql://root:root@127.0.0.1:13306/todo_app";
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(4)
        .build(manager)
        .map_err(|error| panic!(error))
        .unwrap()
}

#[get("/")]
async fn index(context: web::Data<Context>) -> impl Responder {
    let pool = context.pool.get().unwrap();
    let users = web::block(|| find_users(pool)).await.map_err(|e| e);

    if users.is_err() {
        return Err(HttpResponse::BadRequest());
    }
    let responses = users
        .unwrap()
        .iter()
        .map(|user| UserResponse::from(user))
        .collect::<Vec<UserResponse>>();
    Ok(HttpResponse::Ok().json(&responses))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv::dotenv().ok();
    let bind = "127.0.0.1:8080";
    let context = web::Data::new(Context { pool: new_pool() });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(context.clone())
            .service(index)
    })
    .bind(&bind)?
    .run()
    .await
}
