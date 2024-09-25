use crate::tri_state::TriStateController;

pub use map::{SignalId, SwitchId, TrackId};
use serde::{Deserialize, Serialize};
pub use state::RailwayState;

mod map;
mod state;

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
    pub is_right: TriStateController,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct SignalState {
    pub is_clear: TriStateController,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    track: TrackId,
    remaining_length: u64,
}

impl SwitchState {
    pub fn direction(&self) -> Direction {
        match self.is_right.state() {
            true => Direction::Right,
            false => Direction::Left,
        }
    }

    pub fn stable_direction(&self) -> Option<Direction> {
        match self.is_right.tri_state() {
            Some(true) => Some(Direction::Right),
            Some(false) => Some(Direction::Left),
            None => None,
        }
    }
}
