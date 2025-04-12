mod routes;
mod models;

use actix_web::{web, App, HttpServer, cookie::Key, cookie::SameSite};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};

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

    let secret_key = Key::generate();
    

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_origin("http://localhost:8080") // Your frontend URL
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allow_headers(vec!["Content-Type"])
            .supports_credentials();

        let cookie_store = CookieSessionStore::default();

        App::new()
            .wrap(cors)
            .wrap(SessionMiddleware::builder(
                    cookie_store,
                    secret_key.clone(),
                )
                .cookie_secure(false)
                .cookie_same_site(SameSite::Lax)
                .build()
            )
            .app_data(web::Data::new(pg_pool.clone()))
            .configure(config)    
        })
    .bind(&address)?
    .run()
    .await
}
