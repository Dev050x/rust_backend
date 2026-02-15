use crate::auth::models::{ApiResponse, Claims, SignInApiRespnse, Signup, User};
use actix_web::{HttpRequest, HttpResponse, Responder, post, web, get};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp};
use sqlx::PgPool;

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

    match user {
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
    dotenvy::dotenv().ok();
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
        Ok(u) => {
            let secret = std::env::var("JWT_SECRET").unwrap_or("super_secret_key".to_string());

            let claim = Claims {
                sub: u.id,
                username: u.username,
                exp: get_current_timestamp()  + 60 * 60 * 24
            };

            match encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_bytes())) {

                Ok(token) => HttpResponse::Ok().json(SignInApiRespnse{
                    status: 200,
                    message: String::from("user logged in"),
                    token: token
                }),
                Err(_) => HttpResponse::InternalServerError().json(ApiResponse {
                    status: 500,
                    message: String::from("Failed to generate token"),
                }),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(ApiResponse {
            status: 401,
            message: String::from("Invalid username or password"),
        }),
    }
}


#[get("/dummy")]
pub async fn dummy(req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {
    dotenvy::dotenv().ok();

    let auth_header = req.headers().get("Authorization");

    let token = match auth_header {
        Some(value) => {
            let header_str = value.to_str().unwrap_or("");
            if header_str.starts_with("Bearer ") {
                header_str[7..].to_string()
            } else {
                return HttpResponse::Unauthorized().json(ApiResponse {
                    status: 401,
                    message: String::from("Invalid format. Use: Bearer <token>"),
                });
            }
        }
        None => {
            return HttpResponse::Unauthorized().json(ApiResponse {
                status: 401,
                message: String::from("Missing Authorization header"),
            });
        }
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or("super_secret_key".to_string());

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => {
            let user = sqlx::query_as!(
                User,
                "SELECT id, username, password FROM users WHERE id = $1",
                data.claims.sub
            )
            .fetch_one(pool.get_ref())
            .await;

            match user {
                Ok(u) => HttpResponse::Ok().json(u),
                Err(_) => HttpResponse::NotFound().json(ApiResponse {
                    status: 404,
                    message: String::from("User not found"),
                }),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(ApiResponse {
            status: 401,
            message: String::from("Invalid or expired token"),
        }),
    }
}
