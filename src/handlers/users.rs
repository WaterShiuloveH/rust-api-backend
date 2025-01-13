use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use crate::entities::users;
use crate::entities::users::Entity as User;

#[derive(serde::Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

// Create a new user
pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let user = users::ActiveModel {
        name: Set(new_user.name.clone()),
        email: Set(new_user.email.clone()),
        ..Default::default()
    };

    let result = user.insert(db.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

// Get all users
pub async fn get_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    let users = User::find().all(db.get_ref()).await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

// Get a user by ID
pub async fn get_user_by_id(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let user = User::find_by_id(user_id.into_inner()).one(db.get_ref()).await;
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

// Update a user
pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>,
    updated_user: web::Json<NewUser>,
) -> impl Responder {
    let user = User::find_by_id(user_id.into_inner()).one(db.get_ref()).await;
    if let Ok(Some(user)) = user {
        let mut user: users::ActiveModel = user.into();
        user.name = Set(updated_user.name.clone());
        user.email = Set(updated_user.email.clone());

        let result = user.update(db.get_ref()).await;
        match result {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
        }
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

// Delete a user
pub async fn delete_user(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let result = User::delete_by_id(user_id.into_inner()).exec(db.get_ref()).await;
    match result {
        Ok(delete_result) => {
            if delete_result.rows_affected > 0 {
                HttpResponse::Ok().body("User deleted")
            } else {
                HttpResponse::NotFound().body("User not found")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}
