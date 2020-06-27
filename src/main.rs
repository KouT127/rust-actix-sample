pub mod actions;
pub mod models;

use crate::actions::{create_user, find_users};
use crate::models::{NewUser, UserResponse};
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use chrono::Utc;
use sqlx::{MySqlConnection, MySqlPool, Pool};
use std::env;
use tera::Tera;

pub struct Context {
    pool: Pool<MySqlConnection>,
    template: Tera,
}

pub async fn new_pool() -> MySqlPool {
    let host = std::env::var("DATABASE_HOST").expect("Not exists host");
    let port = std::env::var("DATABASE_PORT").expect("Not exists host");
    let user = std::env::var("DATABASE_USER").expect("Not exists host");
    let password = std::env::var("DATABASE_PASSWORD").expect("Not exists host");
    let name = std::env::var("DATABASE_NAME").expect("Not exists host");

    let url = format!(
        "mysql://{user}:{pass}@{host}:{port}/{name}",
        user = user,
        pass = password,
        host = host,
        port = port,
        name = name,
    );

    MySqlPool::builder()
        .max_size(5)
        .build(&url)
        .await
        .expect("Failed to mysql")
}

#[get("/users")]
async fn fetch_users_handler(context: web::Data<Context>) -> impl Responder {
    let pool = context.pool.clone();
    let users = find_users(&pool).await;

    if users.is_err() {
        return Err(HttpResponse::BadRequest());
    }
    let responses = users
        .unwrap()
        .iter()
        .map(|user| UserResponse::from_user(&user))
        .collect::<Vec<UserResponse>>();
    Ok(HttpResponse::Ok().json(&responses))
}

#[post("/users")]
async fn create_user_handler(context: web::Data<Context>) -> impl Responder {
    let pool = &context.pool;
    let mut user = NewUser {
        id: None,
        name: "test".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let result = create_user(pool, &mut user).await;
    if result.is_err() {
        return Err(HttpResponse::BadRequest());
    }

    let responses = UserResponse::from_new_user(&user);
    Ok(HttpResponse::Ok().json(&responses))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv::dotenv().ok();
    let bind = "127.0.0.1:8080";
    let templates = Tera::new("templates/**/*").unwrap();
    let new_pool = new_pool().await;
    let context = web::Data::new(Context {
        pool: new_pool,
        template: templates,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(context.clone())
            .service(fetch_users_handler)
            .service(create_user_handler)
    })
    .workers(2)
    .bind(&bind)?
    .run()
    .await
}
