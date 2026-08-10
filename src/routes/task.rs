use axum::{Json, extract::{Path, State}, http::StatusCode};

use crate::{AppState, error::AppError, models::task::{Task, CreateTask, UpdateTask}};

pub async fn create_task(
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

pub async fn list_tasks(
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
pub async fn get_task(
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

pub async fn update_task(
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

pub async fn delete_task (
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