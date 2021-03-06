extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use handler::UserHandler;
use model::context::{Context, Handler};
use repository::{get_url_from_env, new_pool};
use std::env;
use tera::Tera;

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
    env::set_var(
        "RUST_LOG",
        "rust-actix-sample=debug,actix_web=info,actix_server=info",
    );
    dotenv::dotenv().ok();
    env_logger::init();
    let url = "127.0.0.1:8080";
    let templates = Tera::new("templates/**/*").unwrap();
    let database_url = get_url_from_env();
    let pool = new_pool(database_url, 4);
    let context = web::Data::new(Context {
        pool,
        template: templates,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(context.clone())
            .service(web::resource("/users").route(web::get().to(sample_template)))
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/v1")
                    .service(
                        web::resource("/users")
                            .route(web::post().to(Handler::create_user_handler))
                            .route(web::get().to(Handler::get_users_handler)),
                    )
                    .service(
                        web::resource("/users/{user_id}")
                            .route(web::put().to(Handler::update_user_handler)),
                    ),
            )
    })
    .workers(1)
    .bind(&url)?
    .run()
    .await
}
