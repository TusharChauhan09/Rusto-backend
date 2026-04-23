use axum::{Router, routing::get};
use crate::handlers::user_handler::{sign_in, sign_up};

pub fn user_routes() -> Router {
    Router::new()
        .route("/sign-up", get(sign_up))
        .route("/sign-in", get(sign_in))
}