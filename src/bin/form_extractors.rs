use axum::{
    extract::Form,
    routing::{get, post},
    response::Html,
    Router,
};
use serde::Deserialize;

// 1. Define the struct matching the input fields in your HTML form
#[derive(Deserialize, Debug)]
struct CreateUser {
    username: String,
    email: String,
}

// 2. The Handler Function
// Axum automatically extracts the urlencoded form data into `CreateUser`
async fn handle_form(Form(payload): Form<CreateUser>) -> String {
    format!(
        "Form received! Username: {}, Email: {}",
        payload.username, payload.email
    )
}

// Handler to render a simple HTML page with a form
async fn show_form() -> Html<&'static str> {
    Html(r#"
        <form action="/submit" method="post">
            <label>Name: <input type="text" name="username"></label><br>
            <label>Email: <input type="email" name="email"></label><br>
            <button type="submit">Submit</button>
        </form>
    "#)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/form", get(show_form))
        .route("/submit", post(handle_form));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000/form");
    axum::serve(listener, app).await.unwrap();
}