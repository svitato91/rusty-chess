use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use std::env;
use actix_session::config::CookieContentSecurity::Private;
use actix_session::{Session, SessionGetError, SessionMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use crate::session::Players;

mod chess;
mod session;

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
            .service(me)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello));
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

#[get("/me")]
async fn me(session: Session, data: web::Data<Players>) -> HttpResponse {
    let players = data.get_ref();
    let session_id = session.get::<u64>("id");
    match session_id {
        Ok(Some(id)) => if players.contains(id) {
            HttpResponse::Ok().body(format!("User id: {}", id))
        } else {
            add_player(session, players)
        },
        Ok(None) => add_player(session, players),
        Err(e) => HttpResponse::InternalServerError().
            body(format!("Failed to set user id {}", e.to_string()))
    }
}

fn add_player(session: Session, players: &Players) -> HttpResponse {
    match session.insert("id", players.new_player()) {
        Ok(_) => HttpResponse::Ok().body("User id set"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to set user id")
    }
}

#[get("/")]
async fn hello(data: web::Data<chess::ChessGame>) -> impl Responder {
    let game = data.get_ref();
    println!("{}", game.board_terminal());
    HttpResponse::Ok().json(game.board())
}

#[get("/online")]
async fn echo(data: web::Data<Players>) -> impl Responder {
    HttpResponse::Ok().json(data.get_ref().player_list())
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}