use crate::models::{user_model::{UserSignupBody}};

use axum::{body, extract::{Json,Path,Query}};

use serde::{Serialize,Deserialize};


pub async fn sign_up(
    Json(body) : Json<UserSignupBody>
){
    // "signed up"
    let user = body;

}

pub async fn sign_in() -> &'static str {
    "signed in"
}