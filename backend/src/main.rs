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

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
	    CookieSessionStore::default(), Key::from(&[0; 64])
    )
    .cookie_secure(false)
    .cookie_same_site(SameSite::Lax)
    .cookie_name("auth".to_string())
    .cookie_http_only(true)
	.build() 
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pg_pool = postgres::create_pool();

    let address = address();    

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()// Your frontend URL
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization", "Access-Control-Allow-Credentials"])
            .expose_headers(vec!["Set-Cookie"])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(session_middleware())
            .app_data(web::Data::new(pg_pool.clone()))
            .configure(config)    
        })
    .bind(&address)?
    .run()
    .await
}
