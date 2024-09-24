use std::marker::PhantomData;

use anyhow::Result;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use tokio::select;
use tracing::warn;

use super::messages::{IncomingMessage, OutgoingMessage, SocketError};
use crate::{state::ServerState, Game};

pub struct Socket<G>(WebSocket, PhantomData<G>);

impl<G: Game> Socket<G> {
    async fn send(&mut self, message: OutgoingMessage<G>) -> Result<()> {
        let payload = serde_json::to_string(&message)?;
        self.0.send(Message::Text(payload)).await?;
        Ok(())
    }

    async fn recv(&mut self) -> Result<Option<Result<IncomingMessage<G>, SocketError>>> {
        loop {
            let Some(message) = self.0.recv().await.transpose()? else {
                break Ok(None);
            };

            match message {
                Message::Text(payload) => {
                    match serde_json::from_str(&payload) {
                        Ok(message) => break Ok(Some(Ok(message))),
                        Err(_error) => {
                            break Ok(Some(Err(SocketError::MalformedMessage)));
                        }
                    };
                }
                Message::Binary(_) => {
                    break Ok(Some(Err(SocketError::MalformedMessage)));
                }
                _ => {}
            }
        }
    }
}

pub async fn run<G: Game>(state: ServerState<G>, ws: WebSocket) -> Result<()> {
    let mut socket: Socket<G> = Socket(ws, PhantomData::default());

    let channel = match socket.recv().await? {
        Some(Ok(IncomingMessage::Login { channel })) => channel,
        Some(Ok(_)) => {
            socket
                .send(OutgoingMessage::Error {
                    error: SocketError::BadLogin,
                })
                .await?;

            return Ok(());
        }
        Some(Err(error)) => {
            socket.send(OutgoingMessage::Error { error }).await?;
            return Ok(());
        }
        None => return Ok(()),
    };

    let mut subscription = state.subscriptions.subscribe(channel);

    loop {
        select! {
            message = socket.recv() => match message? {
                Some(Ok(IncomingMessage::Ping)) => {
                    socket.send(OutgoingMessage::Ping).await?;
                }
                Some(Ok(IncomingMessage::Login { channel: _ })) => {
                    socket.send(OutgoingMessage::Error { error: SocketError::BadLogin }).await?;
                    break;
                }

                Some(Err(error)) => {
                    socket.send(OutgoingMessage::Error { error }).await?;
                    break;
                }
                None => break,
            },

            changed = subscription.inner().changed() => {
                let _ = changed?;
                let state = subscription.inner().borrow_and_update().clone();

                if let Some(state) = state {
                    socket.send(OutgoingMessage::State { state }).await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn socket<G: Game>(
    State(state): State<ServerState<G>>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_failed_upgrade(|error| {
        warn!("WebSocket upgrade failed: {error}");
    })
    .on_upgrade(|ws| async {
        let _ = run(state, ws)
            .await
            .inspect_err(|error| warn!("WebSocket closed due to error: {error}"));
    })
}
