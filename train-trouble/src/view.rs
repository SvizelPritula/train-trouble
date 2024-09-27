use enum_map::EnumMap;
use serde::Serialize;

use crate::{
    railroad::{Direction, SignalId, SwitchId, TrackId, TrainId},
    resources::Resource,
    zones::{ZoneId, ZoneInfo},
    TrainToubleGame,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ZoneView {
    name: &'static str,
    switches: Vec<SwitchView>,
    signals: Vec<SignalView>,
    platforms: Vec<PlatformView>,
    rates: Vec<RateView>,
    balance: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SwitchView {
    id: SwitchId,
    name: &'static str,
    direction: Option<Direction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SignalView {
    id: SignalId,
    name: &'static str,
    clear: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PlatformView {
    trains: Vec<TrainView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TrainView {
    id: TrainId,
    name: &'static str,
    stopped: bool,
    load: Option<EnumMap<Resource, u64>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RateView {
    id: Resource,
    name: &'static str,
    rate: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum View {
    Map {
        occupied: EnumMap<TrackId, bool>,
        crash_cleanup_progress: Option<u64>,
    },
    Zone(ZoneView),
}

pub fn zone(id: ZoneId, game: &TrainToubleGame) -> ZoneView {
    let ZoneInfo {
        signals,
        switches,
        platforms,
        ..
    } = id.info();

    let rates = game.market.rates[id];

    ZoneView {
        name: id.name(),
        switches: switches.iter().map(|&id| switch(id, game)).collect(),
        signals: signals.iter().map(|&id| signal(id, game)).collect(),
        platforms: platforms.iter().map(|&id| platform(id, game)).collect(),
        rates: rates
            .into_iter()
            .map(|(id, rate)| RateView {
                id,
                name: id.name(),
                rate,
            })
            .collect(),
        balance: game.balance,
    }
}

fn switch(id: SwitchId, game: &TrainToubleGame) -> SwitchView {
    SwitchView {
        id,
        name: id.name(),
        direction: game.railway.switches[id].direction.tri_state(),
    }
}

fn signal(id: SignalId, game: &TrainToubleGame) -> SignalView {
    SignalView {
        id,
        name: id.name(),
        clear: game.railway.signals[id].is_clear.tri_state(),
    }
}

fn platform(id: SignalId, game: &TrainToubleGame) -> PlatformView {
    PlatformView {
        trains: game
            .railway
            .trains_at_signal(id)
            .into_iter()
            .map(|(id, stopped)| train(id, stopped, game))
            .collect(),
    }
}

fn train(id: TrainId, stopped: bool, game: &TrainToubleGame) -> TrainView {
    TrainView {
        id,
        name: id.name(),
        stopped,
        load: (stopped && !game.railway.has_crash()).then_some(game.loads[id]),
    }
}
