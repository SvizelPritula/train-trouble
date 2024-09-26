use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use crate::{
    railroad::{Direction, SignalId, SwitchId, TrackId, TrainId},
    zones::ZoneId,
    TrainToubleGame,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZoneView {
    switches: Vec<SwitchView>,
    signals: Vec<SignalView>,
    platforms: Vec<PlatformView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SwitchView {
    id: SwitchId,
    direction: Option<Direction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignalView {
    id: SignalId,
    clear: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlatformView {
    trains: Vec<TrainView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrainView {
    id: TrainId,
    stopped: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum View {
    Map { occupied: EnumMap<TrackId, bool> },
    Zone(ZoneView),
}

pub fn zone(id: ZoneId, game: &TrainToubleGame) -> ZoneView {
    let info = id.info();

    ZoneView {
        switches: info.switches.iter().map(|&id| switch(id, game)).collect(),
        signals: info.signals.iter().map(|&id| signal(id, game)).collect(),
        platforms: info
            .platforms
            .iter()
            .map(|&id| platform(id, game))
            .collect(),
    }
}

fn switch(id: SwitchId, game: &TrainToubleGame) -> SwitchView {
    SwitchView {
        id,
        direction: game.railway.switches[id].direction.tri_state(),
    }
}

fn signal(id: SignalId, game: &TrainToubleGame) -> SignalView {
    SignalView {
        id,
        clear: game.railway.signals[id].is_clear.tri_state(),
    }
}

fn platform(id: SignalId, game: &TrainToubleGame) -> PlatformView {
    PlatformView {
        trains: game
            .railway
            .trains_at_signal(id)
            .into_iter()
            .map(|(id, stopped)| TrainView { id, stopped })
            .collect(),
    }
}
