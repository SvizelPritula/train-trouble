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
use crate::{state::ServerState, ActionResult, Game};

pub struct Socket<'a, G>(&'a mut WebSocket, PhantomData<G>);

impl<'a, G: Game> Socket<'a, G> {
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

pub async fn run<G: Game>(state: ServerState<G>, ws: &mut WebSocket) -> Result<()> {
    let mut socket: Socket<G> = Socket(ws, PhantomData);

    macro_rules! socket_error {
        ($error: expr) => {{
            let error = $error;

            warn!("Client made a protocol error: {error:?}");
            socket.send(OutgoingMessage::Error { error }).await?;

            return Ok(());
        }};
    }

    let channel = match socket.recv().await? {
        Some(Ok(IncomingMessage::Login { channel })) => channel,
        Some(Ok(_)) => socket_error!(SocketError::NoLogin),
        Some(Err(error)) => socket_error!(error),
        None => return Ok(()),
    };

    let mut subscription = state.subscriptions.subscribe(channel.clone());
    subscription.inner().mark_changed();

    loop {
        select! {
            message = socket.recv() => match message? {
                Some(Ok(IncomingMessage::Ping)) => socket.send(OutgoingMessage::Ping).await?,
                Some(Ok(IncomingMessage::Login { channel: _ })) => socket_error!(SocketError::DoubleLogin),
                Some(Ok(IncomingMessage::Submit { id, action })) => {
                    let result = state.actions.submit(channel.clone(), action).await?;

                    match result {
                        ActionResult::Ok => socket.send(OutgoingMessage::Confirm { id, error: None }).await?,
                        ActionResult::Error(error) => socket.send(OutgoingMessage::Confirm { id, error: Some(error) }).await?,
                        ActionResult::Misdirected => socket_error!(SocketError::MisdirectedAction),
                    };
                }

                Some(Err(error)) => socket_error!(error),
                None => break,
            },

            changed = subscription.inner().changed() => {
                changed?;
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
    .on_upgrade(|mut ws| async move {
        let _ = run(state, &mut ws)
            .await
            .inspect_err(|error| warn!("WebSocket closed due to error: {error}"));

        let _ = ws.close().await;
    })
}
