use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_db() -> PgPool {
    dotenvy::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();

    println!("Database connected and migrated!");
    pool
}
