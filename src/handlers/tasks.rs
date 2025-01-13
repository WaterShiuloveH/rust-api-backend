use crate::entities::tasks;
use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, IntoActiveModel, ModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewTask {
   pub  title: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
struct TaskResponse {
    id: i32,
    title: String,
    description: Option<String>,
    created_at: String,
}

pub async fn get_tasks(db: web::Data<DbConn>) -> impl Responder {
    let tasks = tasks::Entity::find().all(db.get_ref()).await.unwrap();

    let response: Vec<TaskResponse> = tasks
        .into_iter()
        .map(|task| TaskResponse {
            id: task.id,
            title: task.title,
            description: task.description,
            created_at: task.created_at,
        })
        .collect();

    HttpResponse::Ok().json(response)
}

pub async fn get_task_by_id(db: web::Data<DbConn>, path: web::Path<(i32,)>) -> impl Responder {
    let task: Option<_> = tasks::Entity::find_by_id(path.into_inner().0)
        .one(db.get_ref())
        .await
        .unwrap();
    match task {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().body("Task not found"),
    }
}

pub async fn create_task(db: web::Data<DbConn>, new_task: web::Json<NewTask>) -> impl Responder {
    let task = tasks::ActiveModel {
        title: Set(new_task.title.clone()),
        description: Set(new_task.description.clone()),
        ..Default::default()
    };

    let created_task = task.insert(db.get_ref()).await.unwrap();
    HttpResponse::Created().json(created_task)
}

pub async fn update_task(
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

pub async fn delete_task(db: web::Data<DbConn>, path: web::Path<(i32,)>) -> impl Responder {
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
