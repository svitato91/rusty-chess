use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use futures::{executor, SinkExt, StreamExt};
use futures::executor::LocalPool;
use futures::task::SpawnExt;
use async_tungstenite::async_std::connect_async;
use async_tungstenite::tungstenite::Message;

pub(super) fn handle_connection(sender: Sender<GameStatus>) {
    let thread_pool = AsyncComputeTaskPool::get();
    loop {
        info!("Connecting...");
        let connection = executor::block_on(thread_pool.spawn(connect_async("ws://localhost:2564/socket")));
        match connection {
            Ok((stream, _)) => {
                let (mut write, mut read) = stream.split();
                let mut pool = LocalPool::new();
                let spawner = pool.spawner();

                if let Err(err) = spawner.spawn(async move {
                    while let Some(message) = read.next().await {
                        info!("{message:?}");
                    }
                    info!("Closed read socket");
                }) {
                    error!("Spawning read routine error: {err}");
                    continue;
                }
                if let Err(err) = spawner.spawn(async move {
                    loop {
                        info!("Sending ping...");
                        if let Err(err) = write.send(Message::Ping(vec![123])).await {
                            error!("Error sending message: {err}");
                            break;
                        }
                        thread::sleep(Duration::from_secs(5));
                    }
                }) {
                    error!("Spawning write routine error: {err}");
                    continue;
                }
                pool.run();
                warn!("Connection lost...");
            }
            Err(error) => error!("Connection error {error}"),
        }
    }
}

pub(crate) enum GameStatus {
    Init,
    Connected(Status),
    Disconnected(u64),
}

pub(crate) struct Status {

}
