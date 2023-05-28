//! # Chess server
#![deny(clippy::correctness)]

#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::suspicious)]
#![warn(clippy::style)]
#![warn(missing_docs)]

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::exhaustive_enums)]
#![allow(clippy::exhaustive_structs)]
#![allow(clippy::std_instead_of_core)]
#![allow(clippy::std_instead_of_alloc)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::implicit_return)]
#![allow(clippy::str_to_string)]
#![allow(clippy::shadow_reuse)]
#![allow(clippy::assertions_on_result_states)]
#![allow(clippy::pattern_type_mismatch)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::self_named_module_files)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::separated_literal_suffix)]

use actix_web::{App, HttpResponse, HttpServer, web};
use std::env;
use std::io::ErrorKind;
use actix_session::config::CookieContentSecurity::Private;
use actix_session::{Session, SessionMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use crate::errors::Error;
use crate::players::Players;
use crate::rest::users::user_config;

pub(crate) mod errors;
mod chess;
mod players;
mod games;
mod rest;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = get_secret_key();
    env_logger::init();

    // Players data
    let players = web::Data::new(Players::new());

    let key = secret_key
        .map_err(|e|
            std::io::Error::new(
                ErrorKind::Other,
                format!("Secret key not set up: {e:?}")
        ))?;
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    key.clone(),
                )
                    .cookie_content_security(Private)
                    .build()
            )
            .app_data(players.clone())
            .configure(user_config)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

fn get_secret_key() -> Result<Key, Error> {
    let string = env::var("SECRET_KEY")
        .map_err(Error::Env)?;
    Ok(Key::from(string.as_bytes()))
}


fn add_player(session: &Session, players: &Players) -> HttpResponse {
    match session.insert("id", players.new_player()) {
        Ok(_) => HttpResponse::Ok().body("User id set"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to set user id")
    }
}
