use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserSignupBody {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
}