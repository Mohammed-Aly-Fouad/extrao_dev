use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, version};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[derive(Deserialize, Serialize, Clone)]
struct User {
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {

    users: Arc<Mutex<Vec<User>>>,
    db: PgPool,
}

#[derive(Deserialize)]
struct Pagination {
    page: u32,
    size: u32,
}

#[derive(Deserialize)]
struct PageQuery {
    name: String,
}

#[derive(Deserialize)]
struct UpdateUser {
    name: String,
    email: String,
}

// Handlers
// async fn detail_user(Path(id): Path<i32>) -> impl IntoResponse {
//     format!("User detail for id: {}", id)
// }

async fn detail_user(
    State(app_state): State<AppState>,
    Path(id): Path<i32>) -> impl IntoResponse {
    let (version,): (i32,) = sqlx::query_as("SELECT 1 as version")
    .fetch_one(&app_state.db)
    .await
    .expect("Failed to test connection...");
format!("The version is: {}", version)
}

async fn get_more_than_one(Path((id, name)): Path<(i32, String)>) -> String {
    format!("The id is: {}, and the name is: {}", id, name)
}

async fn index_user(Query(pagination): Query<Pagination>) -> String {
    format!("Pages: {}, and Size: {}", pagination.page, pagination.size)
}

async fn create_user_with_appstate(
    State(app_state): State<AppState>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    println!(">>> ADDING USER TO STATE: {:?}", payload.name); // <-- ADD THIS PRINT
    let mut users = app_state.users.lock().await;
    users.push(payload.clone());

    (StatusCode::CREATED, Json(payload))
}

async fn index_user_with_state(State(app_state): State<AppState>) -> Json<Vec<User>> {
    let users = app_state.users.lock().await;
    Json(users.clone())
}

async fn create_user(Json(payload): Json<User>) -> (StatusCode, String) {
    (
        StatusCode::CREATED,
        format!(
            "User: {} created with email: {}",
            payload.name, payload.email
        ),
    )
}

async fn update_user(
    Path(user_id): Path<u32>,        // 1. Path first
    Query(query): Query<PageQuery>,  // 2. Query second
    Json(payload): Json<UpdateUser>, // 3. Body LAST!
) -> impl IntoResponse {
    format!("Updating user {} with query {}", user_id, query.name)
}

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();
    let databas_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in.env file");

    let db_pool = sqlx::PgPool::connect(&databas_url)
    .await
    .expect("Failed to create pool...");

    let app_state = AppState {
        users: Arc::new(Mutex::new(Vec::new())),
        db: db_pool,
    };

    let app = Router::new()
        .route("/users", get(index_user).post(create_user))
        .route("/users/{id}", get(detail_user))
        .route("/more/{id}/{name}", get(get_more_than_one))
        .route("/with_state", post(create_user_with_appstate))
        .route("/user_state_vec", get(index_user_with_state))
        .route("/update/{user_id}", post(update_user))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
