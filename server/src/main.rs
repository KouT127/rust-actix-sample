extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::Utc;
use diesel::*;
use model::context::Context;
use model::user::{NewUser, User, UserPayload, UserResponse};
use repository::{create_user, create_user2, find_users, new_pool, update_user};
use std::env;
use std::thread::sleep;
use std::time::Duration;
use tera::Tera;

async fn fetch_users_handler(context: web::Data<Context>) -> HttpResponse {
    if let Err(error) = context.pool.get() {
        println!("{}", error);
        return HttpResponse::BadRequest().json("error");
    }
    let users = web::block(move || {
        let pool = context.pool.get().unwrap();

        pool.transaction(|| find_users(&pool))
    })
    .await;

    if let Err(error) = users {
        println!("{}", error);
        return HttpResponse::BadRequest().json("error");
    }
    let responses = users
        .unwrap()
        .iter()
        .map(|user| UserResponse::from_user(&user))
        .collect::<Vec<UserResponse>>();
    HttpResponse::Ok().json(&responses)
}

async fn create_user_handler(
    context: web::Data<Context>,
    payload: web::Json<UserPayload>,
) -> HttpResponse {
    if let Err(error) = context.pool.get() {
        println!("{}", error);
        return HttpResponse::BadRequest().json("error");
    }
    let user = web::block(move || {
        let pool = context.pool.get().unwrap();
        let mut user = NewUser {
            name: payload.name.as_str(),
            created_at: Utc::now().naive_utc(),
            updated_at: Some(Utc::now().naive_utc()),
        };
        pool.transaction(|| create_user(&pool, &mut user))
    })
    .await;

    if let Err(error) = user {
        println!("{}", error);
        return HttpResponse::BadRequest().json("error");
    }

    let responses = UserResponse::from_user(&user.unwrap());
    HttpResponse::Ok().json(&responses)
}

async fn create_user_handler1(
    context: web::Data<Context>,
    payload: web::Json<UserPayload>,
) -> HttpResponse {
    if let Err(error) = context.pool.get() {
        println!("{}", error);
        return HttpResponse::BadRequest().json("error");
    }
    let user = web::block(move || {
        let pool = context.pool.get().unwrap();
        let mut user = NewUser {
            name: payload.name.as_str(),
            created_at: Utc::now().naive_utc(),
            updated_at: Some(Utc::now().naive_utc()),
        };
        pool.transaction(|| create_user2(&pool, &mut user))
    })
    .await;

    if let Err(error) = user {
        println!("{}", error);
        return HttpResponse::BadRequest().json("error");
    }

    let responses = UserResponse::from_user(&user.unwrap());
    HttpResponse::Ok().json(&responses)
}

async fn update_user_handler(
    path: web::Path<u64>,
    context: web::Data<Context>,
    payload: web::Json<UserPayload>,
) -> HttpResponse {
    if let Err(error) = context.pool.get() {
        println!("{}", error);
        return HttpResponse::BadRequest().json("error");
    }
    let pool = context.pool.get().unwrap();
    let user_id = path.to_owned();
    let user = User {
        id: user_id,
        name: payload.name.to_owned(),
        created_at: Utc::now().naive_utc(),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let updated_user = update_user(&pool, &user);
    if let Err(error) = updated_user {
        println!("{}", error);
        return HttpResponse::BadRequest().json("error");
    }

    let responses = UserResponse::from_user(&user);
    HttpResponse::Ok().json(&responses)
}

async fn sample_template(context: web::Data<Context>) -> HttpResponse {
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    dotenv::dotenv().ok();
    env_logger::builder().init();
    let bind = "127.0.0.1:8080";
    let templates = Tera::new("templates/**/*").unwrap();
    let new_pool = new_pool().await;
    let context = web::Data::new(Context {
        pool: new_pool,
        template: templates,
    });

    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .wrap(Logger::default())
            .app_data(context.clone())
            .service(web::resource("/users").route(web::get().to(sample_template)))
            .service(
                web::resource("/v1/users")
                    .route(web::post().to(create_user_handler))
                    .route(web::get().to(fetch_users_handler)),
            )
            .service(web::resource("/v1/users1").route(web::post().to(create_user_handler1)))
            .service(web::resource("/v1/users/{user_id}").route(web::put().to(update_user_handler)))
    })
    .workers(1)
    .bind(&bind)?
    .run()
    .await
}
