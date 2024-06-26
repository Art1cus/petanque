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
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pg_pool.clone()))
            .configure(config)    
        })
    .bind(&address)?
    .run()
    .await
}
