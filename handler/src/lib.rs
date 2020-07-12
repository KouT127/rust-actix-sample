use actix_web::web;
use actix_web::web::Json;
use async_trait::async_trait;
use chrono::Utc;
use diesel::Connection;
use model::context::{Context, Handler, Repository};
use model::user::{FindUsersResponse, NewUser, User, UserPayload, UserResponse};
use repository::UserRepository;

#[async_trait]
pub trait UserHandler {
    async fn get_users_handler(
        context: web::Data<Context>,
    ) -> Result<Json<FindUsersResponse>, actix_web::Error>;

    async fn create_user_handler(
        context: web::Data<Context>,
        payload: web::Json<UserPayload>,
    ) -> Result<Json<UserResponse>, actix_web::Error>;

    async fn update_user_handler(
        path: web::Path<u64>,
        context: web::Data<Context>,
        payload: web::Json<UserPayload>,
    ) -> Result<Json<UserResponse>, actix_web::Error>;
}

#[async_trait]
impl UserHandler for Handler {
    async fn get_users_handler(
        context: web::Data<Context>,
    ) -> Result<Json<FindUsersResponse>, actix_web::Error> {
        let users = web::block(move || {
            let repo = Repository::build(&context.pool)?;
            repo.find_users()
        })
        .await
        .map_err(|error| {
            println!("{}", error);
            return actix_web::error::ErrorBadRequest("error");
        })?;

        let responses = users
            .iter()
            .map(|user| UserResponse::from_user(&user))
            .collect::<Vec<UserResponse>>();

        Ok(Json(FindUsersResponse { responses }))
    }

    async fn create_user_handler(
        context: web::Data<Context>,
        payload: web::Json<UserPayload>,
    ) -> Result<Json<UserResponse>, actix_web::Error> {
        let user = web::block(move || {
            let repo = Repository::build(&context.pool)?;
            let user = NewUser {
                name: payload.name.as_str(),
                created_at: Utc::now().naive_utc(),
                updated_at: Some(Utc::now().naive_utc()),
            };
            repo.conn.transaction(|| repo.create_user(&user))
        })
        .await
        .map_err(|error| {
            println!("{}", error);
            return actix_web::error::ErrorBadRequest("error");
        })?;

        Ok(Json(UserResponse::from_user(&user)))
    }

    async fn update_user_handler(
        path: web::Path<u64>,
        context: web::Data<Context>,
        payload: web::Json<UserPayload>,
    ) -> Result<Json<UserResponse>, actix_web::Error> {
        let user = web::block(move || {
            let repo = Repository::build(&context.pool)?;
            let user_id = path.to_owned();
            let user = User {
                id: user_id,
                name: payload.name.to_owned(),
                created_at: Utc::now().naive_utc(),
                updated_at: Some(Utc::now().naive_utc()),
            };
            repo.update_user(&user)
        })
        .await
        .map_err(|error| {
            println!("{}", error);
            return actix_web::error::ErrorBadRequest("error");
        })?;

        Ok(Json(UserResponse::from_user(&user)))
    }
}
