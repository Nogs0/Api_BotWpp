mod services;
mod repository;
mod api;

use actix_web::{
    web,
    App,
    HttpServer,
};

use dotenv::dotenv;
use sqlx::{
    Postgres,
    Pool,
    postgres::PgPoolOptions
};

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must to be set");
    let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("Conected Successfully");
            pool
        },
        Err(error) =>{
            println!("Conection Failed. Error {error}");
            std::process::exit(1)
        }
    };

    HttpServer::new( move || {
        App::new()
            .app_data(web::Data::new(AppState {db: pool.clone()}))
            .configure(services::config)
    })
    .bind((str::env::var("HOST"), str::env::var("PORT")))?
    .run()
    .await
}  