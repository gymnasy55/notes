#[macro_use]
extern crate diesel;
extern crate dotenv;

mod api;
mod helpers;
mod model;
mod repository;
mod schema;

use crate::{
    api::users::{get_user, get_users},
    repository::users::UsersRepository,
};
use actix_web::{
    middleware::Logger,
    web::{scope, Data},
    App, HttpServer,
};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        let users_repo = UsersRepository::init();
        let users_repo_data = Data::new(users_repo);

        App::new()
            .wrap(logger)
            .app_data(users_repo_data)
            .service(scope("/users").service(get_user).service(get_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
