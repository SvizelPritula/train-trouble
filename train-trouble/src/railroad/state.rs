use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use super::{map::TrainId, Location, SignalId, SignalState, SwitchId, SwitchState, TrackEnding};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RailwayState {
    pub switches: EnumMap<SwitchId, SwitchState>,
    pub signals: EnumMap<SignalId, SignalState>,
    pub trains: EnumMap<TrainId, Location>,
}

impl RailwayState {
    fn trace(&self, mut location: Location, mut distance: u64) -> Location {
        loop {
            if distance < location.remaining_length {
                location.remaining_length -= distance;
                break;
            } else {
                distance -= location.remaining_length;
                location.remaining_length = 0;

                let next_track = match location.track.info().ending {
                    TrackEnding::Track { next } => Some(next),
                    TrackEnding::Switch {
                        switch,
                        left,
                        right,
                    } => Some(if self.switches[switch].is_right.state() {
                        right
                    } else {
                        left
                    }),
                    TrackEnding::Signal { signal, next } => {
                        self.signals[signal].is_clear.state().then_some(next)
                    }
                };

                if let Some(next_track) = next_track {
                    location = Location {
                        track: next_track,
                        remaining_length: next_track.info().length,
                    }
                } else {
                    break;
                }
            }
        }

        location
    }

    fn move_trains(&mut self) {
        let mut trains = self.trains;

        for train in trains.values_mut() {
            let next = self.trace(*train, 1);
            *train = next;
        }

        self.trains = trains;
    }

    pub fn tick(&mut self) {
        self.switches.values_mut().for_each(|s| s.is_right.tick());
        self.signals.values_mut().for_each(|s| s.is_clear.tick());

        self.move_trains();
    }
}

impl Default for RailwayState {
    fn default() -> Self {
        Self {
            switches: Default::default(),
            signals: Default::default(),
            trains: EnumMap::from_fn(|id: TrainId| Location {
                track: id.start(),
                remaining_length: 0,
            }),
        }
    }
}
