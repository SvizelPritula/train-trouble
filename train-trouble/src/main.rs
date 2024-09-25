use std::process::Termination;

use railroad::RailwayState;
use serde::{Deserialize, Serialize};
use train_trouble_engine::{run, ActionResult, Game};

mod railroad;
mod tri_state;
mod zones;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct TrainToubleGame {
    railway: RailwayState,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum Channel {}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum View {}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum Action {}

impl Game for TrainToubleGame {
    type CHANNEL = Channel;
    type VIEW = View;
    type ACTION = Action;

    const TICK_RATE: u64 = 20;

    fn tick(&mut self) {
        self.railway.tick();
    }

    fn view(&mut self, channel: Self::CHANNEL) -> Self::VIEW {
        match channel {}
    }

    fn act(&mut self, channel: Self::CHANNEL, action: Self::ACTION) -> ActionResult {
        match (channel, action) {
            _ => ActionResult::Misdirected,
        }
    }
}

fn main() -> impl Termination {
    run::<TrainToubleGame>()
}
