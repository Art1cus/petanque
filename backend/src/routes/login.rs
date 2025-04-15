use actix_session::Session;
use actix_web::{post, get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub password: String,
}

#[post("/login")]
pub async fn login(info: web::Json<LoginRequest>, session: Session) -> impl Responder {
    let stored_password = "RotaLove2025";

    if info.password == stored_password {
        session.insert("logged_in", true).unwrap();
        HttpResponse::Ok().json("Login successful!")
    } else {
        HttpResponse::Unauthorized().json("Invalid password")
    }
}

#[get("/check_auth")]
pub async fn check_auth(session: Session) -> impl Responder {
    let logged_in = session.get::<bool>("logged_in").unwrap_or(Some(false)).unwrap_or(false);
    if logged_in {
        HttpResponse::Ok().json("Authenticated")
    } else {
        HttpResponse::Unauthorized().json("Not authenticated")
    }
}

#[post("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().json("Logged out")
}