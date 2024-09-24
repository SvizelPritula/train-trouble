use std::{mem::take, sync::Arc};

use parking_lot::Mutex;
use tokio::sync::oneshot;

use crate::{channel_map::ChannelMap, ActionResult, Game};

#[derive(Clone, Default)]
pub struct ServerState<G: Game> {
    pub subscriptions: ChannelMap<G>,
    pub actions: Actions<G>,
}

pub struct Action<G: Game> {
    pub channel: G::CHANNEL,
    pub action: G::ACTION,
    pub sender: oneshot::Sender<ActionResult>,
}

#[derive(Clone, Default)]
pub struct Actions<G: Game>(Arc<Mutex<Vec<Action<G>>>>);

impl<G: Game> Actions<G> {
    pub fn submit(
        &self,
        channel: G::CHANNEL,
        action: G::ACTION,
    ) -> oneshot::Receiver<ActionResult> {
        let (sender, receiver) = oneshot::channel();
        let mut guard = self.0.lock();

        guard.push(Action {
            channel,
            action,
            sender,
        });

        receiver
    }

    pub fn take_actions(&self) -> Vec<Action<G>> {
        take(self.0.lock().as_mut())
    }
}
