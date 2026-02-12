use std::{fmt::format, sync::Mutex};
use actix_web::{App, HttpResponse, HttpServer, Responder, error::HttpError, get, post, web::{self, scope}};
use serde::{Deserialize, Serialize};
struct AppStateWithCounter {
    counter: Mutex<i32>
}

#[get("/")]
async fn hello(data: web::Data<AppStateWithCounter>) ->  String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("the counter is {counter} ")
}

#[get("/users/{user_id}/{friends}")]
async fn index(path: web::Path<(u32, String)>) -> Result<String, HttpError> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[derive(Serialize, Deserialize)]
struct Info {
    name: String,
}


#[post("/user")]
async fn info(info: web::Json<Info>) -> web::Json<Info> {
    web::Json(Info {
        name: info.name.clone()
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
                counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        println!("server is running");
        App::new()
            .service(
                web::scope("/api/v1")
                    .service(hello)
                    .service(index)
                    .service(info)
            )
            .app_data(counter.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}

