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

use actix_web::{App, HttpServer};
use std::env;
use std::io::ErrorKind;
use std::time::Duration;
use actix_rt::time;
use actix_session::config::CookieContentSecurity::Private;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::web::Data;
use log::{debug, info};
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
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    debug!("Loading secret key");
    let key = get_secret_key()
        .map_err(|e|
            std::io::Error::new(
                ErrorKind::Other,
                format!("Secret key not set up: {e:?}")
            ))?;

    debug!("Initializing data");
    // Players data
    let players = Data::new(Players::new());
    // Games data
    let games = Data::new(games::Games::new());

    info!("Starting cleanup routines");
    let players_clone = players.clone();
    actix_rt::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            players_clone.get_ref().cleanup();
        }
    });

    // Start server
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
            .app_data(Data::clone(&players))
            .app_data(Data::clone(&games))
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
