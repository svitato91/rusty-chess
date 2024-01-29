use std::sync::Arc;

use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use axum::Router;
use axum::routing::get;

use crate::connection::handle_connection;
use crate::error::Result;
use crate::state::AppState;

mod error;
mod connection;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/socket", get(handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2564").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_connection)
}
