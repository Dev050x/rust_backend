use crate::config::db::init_db;
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

mod config;

#[derive(Deserialize, Debug)]
struct Signup {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct User {
    id: i32,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct ApiResponse {
    status: i32,
    message: String,
}

#[post("/signup")]
pub async fn singup(pool: web::Data<PgPool>, data: web::Json<Signup>) -> impl Responder {
    println!("request from the user {:?}", data.username);
    println!("user sing up succefully");

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
pub async fn singin(pool: web::Data<PgPool>, data: web::Json<Signup>) -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db().await;

    HttpServer::new(move || {
        println!("server is running");
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1")
                .service(singup)
                .service(singin)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
