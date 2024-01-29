use std::net::TcpStream;
use std::sync::mpsc::Sender;

use bevy::log::error;
use tungstenite::{connect, WebSocket};
use tungstenite::http::Uri;
use tungstenite::stream::MaybeTlsStream;

use crate::error::Result;

type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

pub(super) fn handle_connection(sender: Sender<GameStatus>) {
    loop {
        let socket = match create_connection() {
            Ok(socket) => socket,
            Err(err) => {
                error!("Unable to connect with back-end: {err}");
                continue;
            }
        };
    }
}

pub(super) fn create_connection() -> Result<Socket> {
    Ok(connect(Uri::from_static("ws://localhost:2564/socket"))?.0)
}

pub(crate) enum GameStatus {
    Init,
    Connected(Status),
    Disconnected(u64),
}

pub(crate) struct Status {

}
