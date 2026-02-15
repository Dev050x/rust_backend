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

#[derive(Serialize)]
pub struct SignInApiRespnse{
    pub status: i32,
    pub message: String,
    pub token : String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub exp: u64,
}

