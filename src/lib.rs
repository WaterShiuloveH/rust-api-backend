mod entities;
pub mod handlers;

#[cfg(test)]
mod tests {
    use crate::handlers::tasks::{
        create_task, delete_task, get_task_by_id, get_tasks, update_task,
    };

    use actix_web::{http::StatusCode, test, web, App};
    use sea_orm::ConnectionTrait;
    use sea_orm::{Database, DatabaseBackend, DatabaseConnection, Statement};
    use serde_json::json;

    async fn init_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();

        // Create the schema
        let create_tasks_table = r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        "#;

        db.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            create_tasks_table.to_string(),
        ))
        .await
        .expect("Failed to create tasks table");

        db
    }

    #[tokio::test]
    async fn test_create_task() {
        let db = init_test_db().await;

        let app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(db))
                .route("/tasks", actix_web::web::post().to(create_task)),
        )
        .await;

        let task = serde_json::json!({
            "title": "New Task",
            "description": "Test task description",
            "status": "pending"
        });

        let req = test::TestRequest::post()
            .uri("/tasks")
            .set_json(&task)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[tokio::test]
    async fn test_get_tasks() {
        let db = init_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db.clone()))
                .route("/tasks", web::get().to(get_tasks)),
        )
        .await;

        let req = test::TestRequest::get().uri("/tasks").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(response_json.is_array());
    }

    #[tokio::test]
    async fn test_get_task_by_id() {
        let db = init_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db.clone()))
                .route("/tasks", web::post().to(create_task))
                .route("/tasks/{id}", web::get().to(get_task_by_id)),
        )
        .await;

        let new_task = json!({
            "title": "Test Task",
            "description": "Test Description"
        });

        let req = test::TestRequest::post()
            .uri("/tasks")
            .set_json(&new_task)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let body = test::read_body(resp).await;
        let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let task_id = response_json["id"].as_i64().unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/tasks/{}", task_id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(response_json["id"], task_id);
    }

    #[tokio::test]
    async fn test_update_task() {
        let db = init_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db.clone()))
                .route("/tasks", web::post().to(create_task))
                .route("/tasks/{id}", web::put().to(update_task)),
        )
        .await;

        let new_task = json!({
            "title": "Original Task",
            "description": "Original Description"
        });

        let req = test::TestRequest::post()
            .uri("/tasks")
            .set_json(&new_task)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let body = test::read_body(resp).await;
        let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let task_id = response_json["id"].as_i64().unwrap();

        let updated_task = json!({
            "title": "Updated Task",
            "description": "Updated Description"
        });

        let req = test::TestRequest::put()
            .uri(&format!("/tasks/{}", task_id))
            .set_json(&updated_task)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(response_json["title"], "Updated Task");
        assert_eq!(response_json["description"], "Updated Description");
    }

    #[tokio::test]
    async fn test_delete_task() {
        let db = init_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db.clone()))
                .route("/tasks", web::post().to(create_task))
                .route("/tasks/{id}", web::delete().to(delete_task)),
        )
        .await;

        let new_task = json!({
            "title": "Test Task",
            "description": "Test Description"
        });

        let req = test::TestRequest::post()
            .uri("/tasks")
            .set_json(&new_task)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let body = test::read_body(resp).await;
        let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let task_id = response_json["id"].as_i64().unwrap();

        let req = test::TestRequest::delete()
            .uri(&format!("/tasks/{}", task_id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }
}
