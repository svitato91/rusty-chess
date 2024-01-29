use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ClientRequests {
    NewPlayer,
    Reconnection(u64),
    KeepAlive,
}

#[derive(Serialize, Deserialize)]
pub enum ServerAnswers {
    AssignId(u64),
    KeepAlive,
}
