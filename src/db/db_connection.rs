use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn connection_pool() -> Result<PgPool, sqlx::Error>{
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await? ;

    println!("Connected to DB");

    Ok(pool)
}