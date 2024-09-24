use crate::{channel_map::ChannelMap, Game};

#[derive(Clone)]
pub struct State<G: Game> {
    pub subscriptions: ChannelMap<G>,
}

impl<G: Game> State<G> {
    pub fn new() -> State<G> {
        State {
            subscriptions: ChannelMap::new(),
        }
    }
}
