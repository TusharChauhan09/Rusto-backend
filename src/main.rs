use axum::{Router, routing::get};
use tokio::net::TcpListener;
use dotenvy;
use sqlx::{PgPool,migrate::Migrator};


mod routes;
mod handlers;
mod db;
mod models;

use routes::user_route::user_routes;
use db::db_connection;

#[derive(Clone)]  // (important: cloning PgPool does not create new DB connections—it just clones a handle to same pool)
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: Option<String>,
}

static MIGRATOR: Migrator = sqlx::migrate!();

async fn health_check() -> &'static str {
    "Health check passed!"
}

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();

    // ! DB connection
    let pool = db_connection::connection_pool()
        .await
        .expect("DB connection failed");

    // ! DB migration
    MIGRATOR.run(&pool)
        .await
        .expect("Migration failed");

    // create app state
    let state = AppState {
        pool,
        jwt_secret: None,
    };

    // ! Routes
    let app = Router::new()
        .route("/health-check", get(health_check))
        .nest("/user", user_routes())
        .with_state(state);

    // ! Server binding
    let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Failed to bind"); 
    // let listener = match TcpListener::bind("127.0.0.1:3000").await{
    //         Ok(v) => v,
    //         Err(e) => {
    //             println!("Failed to bind: {}", e);
    //         return;
    //     } 
    // };

    // ! Server start
    axum::serve(listener, app)
        .await
        .unwrap();
}