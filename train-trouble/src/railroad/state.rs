use enum_map::EnumMap;

use super::{map::TrainId, Direction, Location, RailwayState, SignalId, TrackEnding, TrackId};

const CRASH_DISTANCE: u64 = 20;
pub const CRASH_CLEANUP_TIME: u64 = 60 * 20;
pub const CRASH_PENALTY: i64 = 50_000;

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
                    } => Some(match self.switches[switch].direction.state() {
                        Direction::Left => left,
                        Direction::Right => right,
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

    fn detect_crash(&self) -> bool {
        for (a_id, a) in self.trains {
            for (b_id, b) in self.trains {
                if a_id == b_id {
                    continue;
                }

                if a.track != b.track {
                    continue;
                }

                if a.remaining_length.abs_diff(b.remaining_length) < CRASH_DISTANCE {
                    return true;
                }
            }
        }

        false
    }

    pub fn cleanup_crash(&mut self) {
        if let Some(remaining) = &mut self.crash_cleanup_remaining {
            if *remaining == 0 {
                self.crash_cleanup_remaining = None;
                self.trains = Self::default_train_locations();
            } else {
                *remaining -= 1;
            }
        }
    }

    pub fn tick(&mut self) {
        self.cleanup_crash();

        self.switches.values_mut().for_each(|s| s.direction.tick());
        self.signals.values_mut().for_each(|s| s.is_clear.tick());

        if !self.has_crash() {
            self.move_trains();

            if self.detect_crash() {
                self.crash_cleanup_remaining = Some(CRASH_CLEANUP_TIME - 1);
            }
        }
    }

    pub fn has_crash(&self) -> bool {
        self.crash_cleanup_remaining.is_some()
    }

    pub fn has_just_crashed(&self) -> bool {
        self.crash_cleanup_remaining == Some(CRASH_CLEANUP_TIME - 1)
    }

    pub fn crash_cleanup_progress(&self) -> Option<u64> {
        self.crash_cleanup_remaining.map(|remaining| {
            CRASH_CLEANUP_TIME.saturating_sub(remaining) * 100 / CRASH_CLEANUP_TIME
        })
    }

    pub fn occupied(&self) -> EnumMap<TrackId, bool> {
        let mut result = EnumMap::from_fn(|_| false);

        for train in self.trains.values() {
            result[train.track] = true;
        }

        result
    }

    pub fn trains_at_signal(&self, signal: SignalId) -> impl Iterator<Item = (TrainId, bool)> {
        let signal_closed = !self.signals[signal].is_clear.state();

        self.trains
            .into_iter()
            .filter(move |(_train, location)| {
                let track_signal = match location.track.info().ending {
                    TrackEnding::Signal { signal, .. } => Some(signal),
                    _ => None,
                };

                track_signal == Some(signal)
            })
            .map(move |(train, location)| (train, signal_closed && location.remaining_length == 0))
    }

    fn default_train_locations() -> EnumMap<TrainId, Location> {
        EnumMap::from_fn(|id: TrainId| Location {
            track: id.start(),
            remaining_length: 0,
        })
    }
}

impl Default for RailwayState {
    fn default() -> Self {
        Self {
            switches: Default::default(),
            signals: Default::default(),
            trains: Self::default_train_locations(),
            crash_cleanup_remaining: None,
        }
    }
}
