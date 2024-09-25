use std::process::Termination;

use enum_map::EnumMap;
use railroad::{RailwayState, TrackId};
use serde::{Deserialize, Serialize};
use train_trouble_engine::{run, ActionResult, Game};
use zones::{SignalView, SwitchView, ZoneId};

mod railroad;
mod tri_state;
mod zones;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct TrainToubleGame {
    railway: RailwayState,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum Channel {
    Map,
    Zone { zone: ZoneId },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum View {
    Map {
        occupied: EnumMap<TrackId, bool>,
    },
    Zone {
        switches: Vec<SwitchView>,
        signals: Vec<SignalView>,
    },
}

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
        match channel {
            Channel::Map => View::Map {
                occupied: self.railway.occupied(),
            },
            Channel::Zone { zone } => View::Zone {
                switches: zone.switches(&self.railway),
                signals: zone.signals(&self.railway),
            },
        }
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
