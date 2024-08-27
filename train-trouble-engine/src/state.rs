use std::{collections::HashMap, sync::Arc};

use parking_lot::{Mutex, MutexGuard};
use tokio::sync::watch;

use crate::Game;

#[derive(Clone)]
pub struct StateHandle<G: Game>(Arc<Mutex<State<G>>>);

pub struct State<G: Game> {
    subscriptions: HashMap<G::CHANNEL, watch::Sender<Option<G::VIEW>>>,
}

pub struct ViewSubscription<G: Game> {
    receiver: watch::Receiver<Option<G::VIEW>>,
    state: StateHandle<G>,
    channel: G::CHANNEL,
}

impl<G: Game> StateHandle<G> {
    pub fn new() -> Self {
        StateHandle(Arc::new(Mutex::new(State {
            subscriptions: HashMap::new(),
        })))
    }

    pub fn subscribe(&self, channel: G::CHANNEL) -> ViewSubscription<G> {
        let mut guard = self.0.lock();

        let receiver = if let Some(sender) = guard.subscriptions.get(&channel) {
            sender.subscribe()
        } else {
            let (sender, receiver) = watch::channel(None);
            guard.subscriptions.insert(channel.clone(), sender);

            receiver
        };

        ViewSubscription {
            receiver,
            state: self.clone(),
            channel,
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, State<G>> {
        self.0.lock()
    }
}

impl<G: Game> State<G> {
    pub fn subscriptions(
        &self,
    ) -> impl Iterator<Item = (&G::CHANNEL, &watch::Sender<Option<G::VIEW>>)> {
        self.subscriptions.iter()
    }
}

impl<G: Game> ViewSubscription<G> {
    pub fn inner(&self) -> &watch::Receiver<Option<G::VIEW>> {
        &self.receiver
    }
}

impl<G: Game> Drop for ViewSubscription<G> {
    fn drop(&mut self) {
        let mut guard = self.state.0.lock();

        if let Some(sender) = guard.subscriptions.get(&self.channel) {
            if sender.receiver_count() <= 1 {
                guard.subscriptions.remove(&self.channel);
            }
        }
    }
}
