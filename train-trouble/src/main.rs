use std::process::Termination;

use enum_map::EnumMap;
use railroad::{Direction, RailwayState, SignalId, SwitchId, TrainId};
use resources::{Market, Resource};
use serde::{Deserialize, Serialize};
use train_trouble_engine::{run, ActionResult, Game};
use view::View;
use zones::ZoneId;

mod railroad;
mod resources;
mod timers;
mod tri_state;
mod view;
mod zones;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct TrainToubleGame {
    railway: RailwayState,
    market: Market,
    loads: EnumMap<TrainId, EnumMap<Resource, u64>>,
    balance: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum Channel {
    Map,
    Zone { zone: ZoneId },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum Action {
    Switch { id: SwitchId, direction: Direction },
    Signal { id: SignalId, clear: bool },
}

impl Game for TrainToubleGame {
    type CHANNEL = Channel;
    type VIEW = View;
    type ACTION = Action;

    const TICK_RATE: u64 = 20;

    fn tick(&mut self) {
        self.railway.tick();
        self.market.tick();
    }

    fn view(&mut self, channel: Self::CHANNEL) -> Self::VIEW {
        match channel {
            Channel::Map => View::Map {
                occupied: self.railway.occupied(),
            },
            Channel::Zone { zone } => View::Zone(view::zone(zone, self)),
        }
    }

    fn act(&mut self, channel: Self::CHANNEL, action: Self::ACTION) -> ActionResult {
        match (channel, action) {
            (Channel::Zone { zone }, Action::Switch { id, direction }) => {
                if zone.info().switches.contains(&id) {
                    self.railway.switches[id].direction.set(direction);
                    ActionResult::Ok
                } else {
                    ActionResult::Misdirected
                }
            }
            (Channel::Zone { zone }, Action::Signal { id, clear }) => {
                if zone.info().signals.contains(&id) {
                    self.railway.signals[id].is_clear.set(clear);
                    ActionResult::Ok
                } else {
                    ActionResult::Misdirected
                }
            }
            _ => ActionResult::Misdirected,
        }
    }
}

fn main() -> impl Termination {
    run::<TrainToubleGame>()
}
