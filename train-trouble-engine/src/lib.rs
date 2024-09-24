use game_loop::run_loop;
use serde::{Deserialize, Serialize};
use server::run_server;
use state::State;
use std::hash::Hash;
use tokio::try_join;

mod channel_map;
mod game_loop;
mod server;
mod state;

pub trait Game: Serialize + for<'de> Deserialize<'de> + Clone + Default + Send {
    type CHANNEL: Serialize + for<'de> Deserialize<'de> + Clone + Eq + Hash + Send + Sync;
    type VIEW: Serialize + for<'de> Deserialize<'de> + Clone + Eq + Send + Sync;
    type ACTION: Serialize + for<'de> Deserialize<'de> + Clone + Eq + Send + Sync;

    const TICK_RATE: u64;

    fn tick(&mut self);
    fn view(&mut self, channel: Self::CHANNEL) -> Self::VIEW;
    fn act(&mut self, channel: Self::CHANNEL, action: Self::ACTION) -> ActionResult;
}

#[derive(Debug, Clone)]
pub enum ActionResult {
    Ok,
    Error(Box<str>),
    Misdirected,
}

#[tokio::main]
pub async fn run<G: Game + 'static>() {
    let state_for_loop = State::<G>::new();
    let state_for_server = state_for_loop.clone();

    try_join!(
        tokio::spawn(async { run_loop(state_for_loop).await }),
        tokio::spawn(async { run_server(state_for_server).await }),
    )
    .unwrap();
}
