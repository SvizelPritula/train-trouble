use std::{collections::HashMap, ops::Deref, sync::Arc};

use parking_lot::{Mutex, MutexGuard};
use tokio::sync::watch;

use crate::Game;

#[derive(Clone)]
pub struct ChannelMap<G: Game>(Arc<Mutex<Subscriptions<G>>>);
pub struct Subscriptions<G: Game>(HashMap<G::CHANNEL, watch::Sender<Option<G::VIEW>>>);

pub struct ViewSubscription<G: Game> {
    receiver: watch::Receiver<Option<G::VIEW>>,
    map: ChannelMap<G>,
    channel: G::CHANNEL,
}

impl<G: Game> ChannelMap<G> {
    pub fn new() -> Self {
        ChannelMap(Arc::new(Mutex::new(Subscriptions(HashMap::new()))))
    }

    pub fn subscribe(&self, channel: G::CHANNEL) -> ViewSubscription<G> {
        let mut guard = self.0.lock();

        let receiver = if let Some(sender) = guard.0.get(&channel) {
            sender.subscribe()
        } else {
            let (sender, receiver) = watch::channel(None);
            guard.0.insert(channel.clone(), sender);

            receiver
        };

        ViewSubscription {
            receiver,
            map: self.clone(),
            channel,
        }
    }

    pub fn subscriptions(&self) -> impl Deref<Target = Subscriptions<G>> + '_ {
        self.0.lock()
    }
}

impl<G: Game> Subscriptions<G> {
    pub fn iter(&self) -> impl Iterator<Item = (&G::CHANNEL, &watch::Sender<Option<G::VIEW>>)> {
        self.0.iter()
    }
}

impl<G: Game> ViewSubscription<G> {
    pub fn inner(&self) -> &watch::Receiver<Option<G::VIEW>> {
        &self.receiver
    }
}

impl<G: Game> Drop for ViewSubscription<G> {
    fn drop(&mut self) {
        let mut guard = self.map.0.lock();

        if let Some(sender) = guard.0.get(&self.channel) {
            if sender.receiver_count() <= 1 {
                guard.0.remove(&self.channel);
            }
        }
    }
}
