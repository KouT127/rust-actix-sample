pub mod actions;
pub mod models;

use crate::actions::{find_users, find_users2};
use crate::models::UserResponse;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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

#[get("/")]
async fn index(context: web::Data<Context>) -> impl Responder {
    let pool = context.pool.clone();
    let users = find_users(&pool).await;

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

#[get("/3")]
async fn index3(context: web::Data<Context>) -> impl Responder {
    let pool = context.pool.clone();
    let users = find_users2(&pool).await;
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

#[get("/2")]
async fn index2(context: web::Data<Context>) -> impl Responder {
    let tmpl = &context.template;
    let mut ctx = tera::Context::new();
    ctx.insert("users", &vec!["test", "test"]);
    let view = tmpl
        .render("index.tera", &ctx)
        .map_err(|_e| HttpResponse::BadRequest());

    HttpResponse::Ok()
        .content_type("text/html")
        .body(view.unwrap())
}

// #[post("/")]
// async fn create_user_handler(context: web::Data<Context>) -> impl Responder {
//     let result = web::block(move || {
//         let pool = &context.pool;
//         create_user(pool, "test")
//     })
//     .await;
//
//     if result.is_err() {
//         return Err(HttpResponse::BadRequest());
//     }
//     let responses = UserResponse::from(&result.unwrap());
//     Ok(HttpResponse::Ok().json(&responses))
// }

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
            .service(index)
            .service(index2)
            .service(index3)
    })
    .workers(2)
    .bind(&bind)?
    .run()
    .await
}
