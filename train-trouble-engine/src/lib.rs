use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub trait Game: Serialize + for<'de> Deserialize<'de> + Clone + Default {
    type CHANNEL: Serialize + for<'de> Deserialize<'de> + Clone + Eq + Hash;
    type VIEW: Serialize + for<'de> Deserialize<'de> + Clone + Eq;
    type ACTION: Serialize + for<'de> Deserialize<'de> + Clone + Eq;

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
