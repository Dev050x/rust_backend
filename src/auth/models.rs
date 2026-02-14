use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Signup {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: i32,
    pub message: String,
}