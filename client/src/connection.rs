use std::sync::mpsc::Sender;
use std::{future, thread};
use std::time::Duration;

use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, block_on};
use futures::{executor, SinkExt, StreamExt};
use futures::task::SpawnExt;
use async_tungstenite::async_std::connect_async;
use async_tungstenite::tungstenite::Message;
use futures::future::Either;
use futures_timer::Delay;

pub(super) fn handle_connection(sender: Sender<GameStatus>) {
    let thread_pool = AsyncComputeTaskPool::get();
    if let Err(err) = sender.send(GameStatus::Init) {
        // Todo handle the shutdown
    }
    loop {
        info!("Connecting...");
        let connection = executor::block_on(thread_pool.spawn(connect_async("ws://localhost:2564/socket")));
        match connection {
            Ok((stream, _)) => {
                let (mut write, mut read) = stream.split();

                let read_task = thread_pool.spawn(async move {
                    info!("Starting read routine...");
                    while let Some(message) = read.next().await {
                        info!("{message:?}");
                    }
                    info!("Closed read socket");
                });
                let write_task = thread_pool.spawn(async move {
                    loop {
                        info!("Sending ping...");
                        if let Err(err) = write.send(Message::Ping(vec![123])).await {
                            error!("Error sending message: {err}");
                            break;
                        }
                        Delay::new(Duration::from_secs(5)).await;
                    }
                });

                let any_task = thread_pool.spawn(futures::future::select(read_task, write_task));
                match block_on(any_task) {
                    Either::Left((_, task)) => { block_on(task.cancel()); }
                    Either::Right((_, task)) => { block_on(task.cancel()); }
                }
                warn!("Connection lost...");
            }
            Err(err) => error!("Connection error {err}"),
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
