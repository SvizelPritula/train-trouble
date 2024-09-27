use std::process::Termination;

use enum_map::EnumMap;
use railroad::{Direction, RailwayState, SignalId, SwitchId, TrainId, CRASH_PENALTY};
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
    balance: i64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum Channel {
    Map,
    Zone { zone: ZoneId },
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum TradeType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum Action {
    Switch {
        id: SwitchId,
        direction: Direction,
    },
    Signal {
        id: SignalId,
        clear: bool,
    },
    Trade {
        action: TradeType,
        train: TrainId,
        resource: Resource,
        amount: u64,
    },
}

impl Game for TrainToubleGame {
    type CHANNEL = Channel;
    type VIEW = View;
    type ACTION = Action;

    const TICK_RATE: u64 = 20;

    fn tick(&mut self) {
        self.railway.tick();
        self.market.tick();

        if self.railway.has_just_crashed() {
            self.balance -= CRASH_PENALTY;
            self.loads = EnumMap::default();
        }
    }

    fn view(&mut self, channel: Self::CHANNEL) -> Self::VIEW {
        match channel {
            Channel::Map => View::Map {
                occupied: self.railway.occupied(),
                crash_cleanup_progress: self.railway.crash_cleanup_progress(),
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
            (
                Channel::Zone { zone },
                Action::Trade {
                    action,
                    train,
                    resource,
                    amount,
                },
            ) => {
                if !self.train_in_zone(train, zone) {
                    ActionResult::Error("Tento vlak zde nestojí".into())
                } else {
                    match action {
                        TradeType::Buy => {
                            if self.buy(zone, train, resource, amount) {
                                ActionResult::Ok
                            } else {
                                ActionResult::Error("Tak velký náklad se na vlak nevejde".into())
                            }
                        }
                        TradeType::Sell => {
                            if self.sell(zone, train, resource, amount) {
                                ActionResult::Ok
                            } else {
                                ActionResult::Error("Ve vlaku tolik této komodity není".into())
                            }
                        }
                    }
                }
            }
            _ => ActionResult::Misdirected,
        }
    }
}

fn main() -> impl Termination {
    run::<TrainToubleGame>()
}
