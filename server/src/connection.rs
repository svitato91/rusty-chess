use std::sync::Arc;

use axum::extract::State;
use axum::extract::ws::WebSocket;
use log::debug;

use crate::state::AppState;

pub(super) async fn handle_connection(
    State(state): State<Arc<AppState>>,
    mut socket: WebSocket,
) {
    debug!("New connection");
    while let Some(msg) = socket.recv().await {
        debug!("{msg:?}");
    }
}
