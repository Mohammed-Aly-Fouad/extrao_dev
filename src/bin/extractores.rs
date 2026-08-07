


use axum::extract::{Path, Query};
use axum::{Json, Router};
use axum::routing::{get, post};  
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    // Path extractor
    async  fn detail_user(Path(id): Path<i32>)-> String {
        format!("User ID is: {}", id)
    }

    // Query extractor
    #[derive(Deserialize)]
    struct Pagination {
        page: u32,
        size: u32,
    }
    async  fn index_users(Query(pagination): Query<Pagination>)-> String {
        format!("Number of pages is : {}, and the size is: {}",pagination.page, pagination.size)
    }


    // Json

    #[derive(Deserialize)]
    struct User {
        name: String,
        email: String,
    }

    async fn create_user(Json(payload): Json<User>) -> String {
        format!("User {} created with email {}", payload.name, payload.email)
    }

    let app: Router = Router::new()
    .route("/pagination", get(index_users))
    .route("/users/{id}", post(detail_user))
    .route("/create", post(create_user));

    let listener = TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

    axum::serve(listener, app)
    .await
    .unwrap();
}
