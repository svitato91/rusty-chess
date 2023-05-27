use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use std::env;
use actix_session::config::CookieContentSecurity::Private;
use actix_session::{Session, SessionMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use crate::players::Players;
use crate::rest::users::user_config;

mod chess;
mod errors;
mod players;
mod games;
mod rest;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = get_secret_key();
    env_logger::init();

    // Players data
    let players = web::Data::new(Players::new());

    HttpServer::new(move || {
        let app = App::new()
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone(),
                )
                    .cookie_content_security(Private)
                    .build()
            )
            .app_data(players.clone())
            .configure(user_config);
        app
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

fn get_secret_key() -> Key {
    let string = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    Key::from(string.as_bytes())
}


fn add_player(session: Session, players: &Players) -> HttpResponse {
    match session.insert("id", players.new_player()) {
        Ok(_) => HttpResponse::Ok().body("User id set"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to set user id")
    }
}