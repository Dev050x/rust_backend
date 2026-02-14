use actix_web::{HttpResponse, Responder, post, web};
use sqlx::PgPool;
use crate::auth::models::{ApiResponse, Signup, User};

#[post("/signup")]
pub async fn signup(pool: web::Data<PgPool>, data: web::Json<Signup>) -> impl Responder {
    println!("request from the user {:?}", data.username);
    println!("user sign up successfully");

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, password)
         VALUES ($1, $2)
         RETURNING id, username, password",
        data.username,
        data.password
    )
    .fetch_one(pool.get_ref())
    .await;

    match user{
        Ok(u) => HttpResponse::Ok().json(ApiResponse {
            status: 200,
            message: format!("User '{}' created successfully", u.username),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse {
            status: 500,
            message: String::from("Failed to create user"),
        }),
    }
}

#[post("/signin")]
pub async fn signin(pool: web::Data<PgPool>, data: web::Json<Signup>) -> impl Responder {
    println!("user trying to singin...");
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, password FROM users WHERE username = $1 AND password = $2",
        data.username,
        data.password
    )
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(u) => HttpResponse::Ok().json(ApiResponse {
            status: 200,
            message: format!("Welcome back, '{}'!", u.username),
        }),
        Err(_) => HttpResponse::Unauthorized().json(ApiResponse {
            status: 401,
            message: String::from("Invalid username or password"),
        }),
    }
}