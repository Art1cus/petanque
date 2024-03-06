mod routes;
mod models;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

use crate::routes::config::config;

mod postgres;

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pg_pool = postgres::create_pool();

    let address = address();
    HttpServer::new(move || {
        let cors = Cors::default() // You can customize CORS options here
            .allow_any_header() // Whitelist everything
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"]) // Whitelist allowed HTTP methods
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pg_pool.clone()))
            .configure(config)    
        })
    .bind(&address)?
    .run()
    .await
}
