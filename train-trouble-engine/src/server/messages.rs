use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum IncomingMessage<G: Game> {
    Ping,
    Login { channel: G::CHANNEL },
    Submit { id: Uuid, action: G::ACTION },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum OutgoingMessage<G: Game> {
    Ping,
    State { state: G::VIEW },
    Confirm { id: Uuid, error: Option<Box<str>> },
    Error { error: SocketError },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SocketError {
    MalformedMessage,
    NoLogin,
    DoubleLogin,
    MisdirectedAction,
}
