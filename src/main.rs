
use axum::routing::{delete, get, patch, post};
use axum::{Router, middleware};
use tokio::net::TcpListener;
mod error;
mod state;
use state::AppState;
mod routes;
use routes::task::{create_task, update_task, delete_task, get_task,list_tasks};
mod layers;
use layers::custom::{middleware_one, middleware_two};
use tower::ServiceBuilder;
mod models;








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
    
    let app_layer = ServiceBuilder::new()
    .layer(middleware::from_fn(middleware_one))
    .layer(middleware::from_fn(middleware_two));
    // .layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
    .route("/tasks", get(list_tasks))
    .route("/tasks", post(create_task))
    .route("/tasks/{id}", get(get_task))
    .route("/tasks/{id}", patch(update_task))
    .route("/tasks/{id}", delete(delete_task))
    .with_state(app_state)
    .layer(app_layer);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
