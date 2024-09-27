use std::ops::Not;

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use crate::tri_state::TriStateController;

pub use map::{SignalId, SwitchId, TrackId, TrainId};
pub use state::CRASH_PENALTY;

mod map;
mod state;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RailwayState {
    pub switches: EnumMap<SwitchId, SwitchState>,
    pub signals: EnumMap<SignalId, SignalState>,
    pub trains: EnumMap<TrainId, Location>,

    pub crash_cleanup_remaining: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
pub enum TrackEnding {
    Track {
        next: TrackId,
    },
    Switch {
        switch: SwitchId,
        left: TrackId,
        right: TrackId,
    },
    Signal {
        signal: SignalId,
        next: TrackId,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct TrackInfo {
    length: u64,
    ending: TrackEnding,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct SwitchState {
    pub direction: TriStateController<Direction>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct SignalState {
    pub is_clear: TriStateController<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    track: TrackId,
    remaining_length: u64,
}

impl Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
