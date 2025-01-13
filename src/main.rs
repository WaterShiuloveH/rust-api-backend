mod entities;
mod handlers;

use crate::handlers::tasks::{create_task, delete_task, get_task_by_id, get_tasks, update_task};
use crate::handlers::users::{create_user, delete_user, get_user_by_id, get_users, update_user};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use env_logger;
use sea_orm::Database;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Connect to the database
    let db = Database::connect(&database_url)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Error connecting to the database: {}", err);
            std::process::exit(1);
        });

    // Start Actix-web server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone())) // Share the database connection
            // Task routes
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks/{id}", web::get().to(get_task_by_id))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks/{id}", web::put().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
            // User routes
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")? // Bind server to localhost
    .run()
    .await
}
