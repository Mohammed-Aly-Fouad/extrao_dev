use axum::{Router, routing::{get, post}};
use tokio::net::TcpListener;

 #[tokio::main]
 async fn main() {

    async fn get_users()-> String {
        String::from("Users Page")
    }

    async fn get_profile()-> String {
        String::from("Profile Page")
    }

    let app_route = Router::new()
    .route("/", get(get_users))
    .route("/profile", get(get(get_profile)));


    let app: Router = Router::new()
    .nest("/v1/users", app_route);
    let listner = TcpListener::bind("0.0.0.0:3000").await
    .unwrap();

    axum::serve(listner, app).await
    .unwrap()
}
