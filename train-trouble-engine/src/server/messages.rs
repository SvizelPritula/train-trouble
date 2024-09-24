use serde::{Deserialize, Serialize};

use crate::Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum IncomingMessage<G: Game> {
    Ping,
    Login { channel: G::CHANNEL },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum OutgoingMessage<G: Game> {
    Ping,
    State { state: G::VIEW },
    Error { error: SocketError },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SocketError {
    MalformedMessage,
    BadLogin,
}
