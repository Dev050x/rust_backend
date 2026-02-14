use crate::{auth::auth_handlers::{signin, signup}, config::db::init_db};
use actix_web::{App, HttpServer, web};

mod config;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db().await;

    HttpServer::new(move || {
        println!("server is running");
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1")
                .service(signup)
                .service(signin)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
