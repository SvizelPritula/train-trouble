use crate::{channel_map::ChannelMap, Game};

#[derive(Clone)]
pub struct ServerState<G: Game> {
    pub subscriptions: ChannelMap<G>,
}

impl<G: Game> ServerState<G> {
    pub fn new() -> ServerState<G> {
        ServerState {
            subscriptions: ChannelMap::new(),
        }
    }
}
