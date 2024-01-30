use std::sync::Arc;
use axum::Error;

use axum::extract::ws::{Message, WebSocket};
use log::{debug, error};

use crate::state::AppState;

pub(super) async fn handle_connection(
    state: Arc<AppState>,
    mut socket: WebSocket,
) {
    debug!("New connection");
    while let Some(message) = socket.recv().await {
        match message {
            Ok(message) => {
                match message {
                    Message::Text(message) => debug!("Received message: {message}"),
                    Message::Binary(message) => debug!("Received message: {message:?}"),
                    Message::Pong(message) => debug!("Received message: {message:?}"),
                    Message::Close(message) => debug!("Received message: {message:?}"),
                    Message::Ping(message) => {
                        debug!("Received ping: {message:?}");
                        if let Err(err) = socket.send(Message::Pong(vec![123])).await {
                            error!("Unable to answer to ping: {err}");
                        }
                    },
                }
            }
            Err(err) => {
                error!("Communication error: {err}");
            }
        }
    }
}
