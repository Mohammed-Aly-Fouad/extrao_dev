use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::Response};

pub async fn middleware_one(req: Request, next: Next) -> Response {
    println!("MIDDLEWARE 1");
    let response = next.run(req).await;
    response

}

pub async fn middleware_two(req: Request, next: Next) -> Response {
    let current = Instant::now();
    let mut response = next.run(req).await;
    let elapsed = format!("{:?}", current.elapsed());
    println!("Latency: {:?}", current.elapsed());
    response.headers_mut().insert("x-response-time", elapsed.parse().unwrap());
    response

}

// pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
//     let auth_header = req.headers().get("Authorization");

//     if auth_header.is_none() {
//         println!("No Authorization");
//         // قطع الطلب وإرجاع خطأ 401 Unauthorized فوراً
//         return Err(StatusCode::UNAUTHORIZED); 
//     }

//     Ok(next.run(req).await)
// }