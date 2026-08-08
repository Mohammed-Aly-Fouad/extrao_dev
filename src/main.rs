use axum::extract::{ Path, State};
use axum::http::{StatusCode};
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use tokio::net::TcpListener;
mod error;
use error::AppError;


#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Serialize)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTask {
    title: Option<String>,
    completed: Option<bool>,
}
async fn create_task(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTask>
) -> Result<Json<Task>, (StatusCode, String)> {
    let data_result = sqlx::query_as!(
        Task,
        r#"INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, completed"#,
        payload.title,
    ).fetch_one(&app_state.db)
    .await;
    
    match data_result {
        Ok(task) => Ok(Json(task)),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Failed to create a task")))
    }
}

async fn list_tasks(
    State(app_state): State<AppState>

) -> Json<Vec<Task>> {
    let data_result = sqlx::query_as!(
        Task,
        r#"SELECT id, title, completed FROM tasks ORDER By id"#
    )
    .fetch_all(&app_state.db).await;

    match data_result {
        Ok(tasks) => Json(tasks),
        Err(_) => Json(Vec::new())
    }
}
async fn get_task(
    State(app_state): State<AppState>,
    Path(id): Path<i32>

) -> Result<Json<Task>, String> {
    let data_result = sqlx::query_as!(
        Task,
        r#"SELECT id, title, completed FROM tasks WHERE id=$1 ORDER By id"#,
        id
    )
    .fetch_one(&app_state.db).await;

    match data_result {
    Ok(task) => Ok(Json(task)),
    Err(_) => Err(String::from("Failed to fetch task")),
}
}

async fn update_task(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTask>

) -> Result<StatusCode, AppError> {
    let updated = sqlx::query!(r#"
    UPDATE tasks SET title = COALESCE($1, title), completed = COALESCE($2, completed) WHERE id = $3
    "#,
    payload.title,
    payload.completed,
    id)
    .execute(&app_state.db)
    .await?;
    
    if updated.rows_affected() > 0 {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("Task with id: {} nto found", id)))
    }

}

async fn delete_task (
    State(app_state): State<AppState>,
    Path(id): Path<i32>
) -> StatusCode {
    let deleted = sqlx::query!(r#"DELETE FROM tasks WHERE id = $1"#, id)
    .execute(&app_state.db)
    .await;
    match deleted {
        Ok(result) => {
            if result.rows_affected() > 0 {
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}
#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();

    let databas_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in.env file");

    let db_pool = sqlx::PgPool::connect(&databas_url)
    .await
    .expect("Failed to create pool...");

    
    let app_state = AppState {
        db: db_pool,
    };
    
    let app = Router::new()
    .route("/tasks", get(list_tasks))
    .route("/tasks", post(create_task))
    .route("/tasks/{id}", get(get_task))
    .route("/tasks/{id}", patch(update_task))
    .route("/tasks/{id}", delete(delete_task))
    .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
