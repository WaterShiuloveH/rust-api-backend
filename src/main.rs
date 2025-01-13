mod entities;
mod handlers; 

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use dotenvy::dotenv;
use env_logger;
use std::env;
use crate::handlers::tasks::{create_task, delete_task, get_task_by_id, get_tasks, update_task};
use sea_orm::Database;

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
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks/{id}", web::get().to(get_task_by_id))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks/{id}", web::put().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
    })
    .bind("127.0.0.1:8080")? // Bind server to localhost
    .run()
    .await
}