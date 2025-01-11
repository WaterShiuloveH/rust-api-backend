# Rust API with Actix-web and SeaORM

This project provides a simple RESTful API for managing tasks using Actix-web, SeaORM, and SQLite as the database. The API allows users to create, read, update, and delete tasks.

## Features
- **Create** a new task.
- **Read** all tasks or a specific task by ID.
- **Update** a task by ID.
- **Delete** a task by ID.

## Requirements
- **Rust**: [Install Rust](https://www.rust-lang.org/learn/get-started)
- **SQLite**: SQLite database to store tasks.
- **dotenv**: Used for environment variable management.

## Setup

### 1. Clone the repository
```bash
git clone https://github.com/your-username/rust-api-backend.git
cd rust-api-backend
```

### 2. Install dependencies
This project uses the following dependencies:

- **actix-web** for building the web server.
- **sea-orm** for ORM and database interaction.
- **dotenvy** for loading environment variables.
To install the dependencies, run:

```bash
cargo build
```
### 3. Configure .env file
Create a .env file in the root of the project with the following content:
```bash
DATABASE_URL=sqlite://path/to/your/db.sqlite
```
You can use a relative path, such as DATABASE_URL=sqlite://db.sqlite, or provide an absolute path.

### 4. Run the server
```bash
cargo run
```
This will start the server at http://127.0.0.1:8080.

## Endpoints

### 1. Get all tasks
```bash
curl http://127.0.0.1:8080/tasks
```

### 2. Get a task by ID
```bash
curl http://127.0.0.1:8080/tasks/1
```

### 3. Create a new task
```bash
curl -X POST http://127.0.0.1:8080/tasks \
     -H "Content-Type: application/json" \
     -d '{"title": "Sample Task", "description": "This is a description of the task."}'
```

### 4. Update a task
```bash
curl -X PUT http://127.0.0.1:8080/tasks/1 \
     -H "Content-Type: application/json" \
     -d '{"title": "Updated Task", "description": "Updated description"}'

```

### 5. Delete a task
```bash
curl -X DELETE http://127.0.0.1:8080/tasks/1
```