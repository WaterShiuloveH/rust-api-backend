use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{ActiveModelTrait, Database, DbConn, EntityTrait, IntoActiveModel, ModelTrait, Set};
use serde::{Deserialize, Serialize};
mod entities;
use actix_web::middleware::Logger;
use entities::tasks;
use env_logger;

#[derive(Deserialize)]
struct NewTask {
    title: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct TaskResponse {
    id: i32,
    title: String,
    description: Option<String>,
    is_completed: bool,
}

async fn get_tasks(db: web::Data<DbConn>) -> impl Responder {
    let tasks = tasks::Entity::find().all(db.get_ref()).await.unwrap();

    let response: Vec<TaskResponse> = tasks
        .into_iter()
        .map(|task| TaskResponse {
            id: task.id,
            title: task.title,
            description: task.description,
            is_completed: task.is_completed,
        })
        .collect();

    HttpResponse::Ok().json(response)
}

async fn get_task_by_id(db: web::Data<DbConn>, path: web::Path<(i32,)>) -> impl Responder {
    let task: Option<_> = tasks::Entity::find_by_id(path.into_inner().0)
        .one(db.get_ref())
        .await
        .unwrap();
    match task {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().body("Task not found"),
    }
}

async fn create_task(db: web::Data<DbConn>, new_task: web::Json<NewTask>) -> impl Responder {
    let task = tasks::ActiveModel {
        title: Set(new_task.title.clone()),
        description: Set(new_task.description.clone()),
        is_completed: Set(false),
        ..Default::default()
    };

    let created_task = task.insert(db.get_ref()).await.unwrap();
    HttpResponse::Created().json(created_task)
}

async fn update_task(
    db: web::Data<DbConn>,
    path: web::Path<(i32,)>,
    updated_task: web::Json<NewTask>,
) -> impl Responder {
    let task = tasks::Entity::find_by_id(path.into_inner().0)
        .one(db.get_ref())
        .await
        .unwrap();

    match task {
        Some(task) => {
            // Convert `Model` to `ActiveModel`
            let mut active_task = task.into_active_model();

            // Update the fields using `Set`
            active_task.title = Set(updated_task.title.clone());
            active_task.description = Set(updated_task.description.clone());

            // Save the updated task
            let updated_task = active_task.update(db.get_ref()).await.unwrap();

            HttpResponse::Ok().json(updated_task)
        }
        None => HttpResponse::NotFound().body("Task not found"),
    }
}

async fn delete_task(db: web::Data<DbConn>, path: web::Path<(i32,)>) -> impl Responder {
    let task = tasks::Entity::find_by_id(path.into_inner().0)
        .one(db.get_ref())
        .await
        .unwrap();

    match task {
        Some(task) => {
            task.delete(db.get_ref()).await.unwrap();
            HttpResponse::NoContent().finish()
        }
        None => HttpResponse::NotFound().body("Task not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Connect to the database
    let db = Database::connect(
        "sqlite:///Users/shiutzfan/Desktop/GeekFolder/rust-api-backend/db.sqlite",
    )
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
