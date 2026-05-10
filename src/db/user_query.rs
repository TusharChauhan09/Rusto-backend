use sqlx::PgPool;

use crate::models::user_model::User;

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password: &str,
    username: &str,
)-> user {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password)
        VALUES ($1, $2, $3)
        RETURNING id, email, username
        "#,
        username,
        password
    )
    .fetch_one(pool)
    .await?;

    ok(user)
}