use axum::{Router, routing::get};
use tokio::net::TcpListener;

mod routes;
mod handlers;

use routes::user_route::user_routes;

async fn health_check() -> &'static str {
    "Health check passed!"
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health-check", get(health_check))
        .nest("/user", user_routes());

    // let listener = TcpListener::bind("127.0.0.1:3000").await().unwrap(); 
    let listener = match TcpListener::bind("127.0.0.1:3000").await{
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to bind: {}", e);
            return;
        } 
    };

    axum::serve(listener, app)
        .await
        .unwrap();
}