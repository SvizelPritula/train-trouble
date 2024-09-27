use enum_map::Enum;
use serde::{Deserialize, Serialize};

use super::{TrackEnding, TrackInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrackId {
    ToNováVesLeft,
    ToNováVesRight,
    NováVesLeftStop,
    NováVesRightStop,
    NováVesLeftSwitch,
    NováVesRightSwitch,

    ToHorníMechoklaty,
    HorníMechoklatyStop,

    ToDolníMechoklaty,
    DolníMechoklatyStop,
    DolníMechoklatySwitch,

    ToPředvořanySwitchLeft,
    ToPředvořanySwitchRight,
    PředvořanySwitch,
    PředvořanyStop,

    ToKolnovLeft,
    ToKolnovRight,
    KolnovEntrySwitch,
    KolnovEntryConnection,
    KolnovRightStop,
    KolnovLeftStop,
    KolnovExitSwitch,
    KolnovExitConnection,
}

impl TrackId {
    pub fn info(self) -> TrackInfo {
        match self {
            TrackId::ToNováVesLeft => TrackInfo {
                length: 160,
                ending: TrackEnding::Signal {
                    signal: SignalId::NováVesLeftEntry,
                    next: TrackId::NováVesLeftStop,
                },
            },
            TrackId::ToNováVesRight => TrackInfo {
                length: 160,
                ending: TrackEnding::Signal {
                    signal: SignalId::NováVesRightEntry,
                    next: TrackId::NováVesRightStop,
                },
            },
            TrackId::NováVesLeftStop => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    signal: SignalId::NováVesLeftExit,
                    next: TrackId::NováVesLeftSwitch,
                },
            },
            TrackId::NováVesRightStop => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    signal: SignalId::NováVesRightExit,
                    next: TrackId::NováVesRightSwitch,
                },
            },
            TrackId::NováVesLeftSwitch => TrackInfo {
                length: 20,
                ending: TrackEnding::Switch {
                    switch: SwitchId::NováVesLeft,
                    left: TrackId::ToHorníMechoklaty,
                    right: TrackId::ToDolníMechoklaty,
                },
            },
            TrackId::NováVesRightSwitch => TrackInfo {
                length: 20,
                ending: TrackEnding::Switch {
                    switch: SwitchId::NováVesRight,
                    left: TrackId::ToHorníMechoklaty,
                    right: TrackId::ToDolníMechoklaty,
                },
            },

            TrackId::ToHorníMechoklaty => TrackInfo {
                length: 200,
                ending: TrackEnding::Signal {
                    signal: SignalId::HorníMechoklatyEntry,
                    next: TrackId::HorníMechoklatyStop,
                },
            },
            TrackId::HorníMechoklatyStop => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    signal: SignalId::HorníMechoklatyExit,
                    next: TrackId::ToPředvořanySwitchLeft,
                },
            },

            TrackId::ToDolníMechoklaty => TrackInfo {
                length: 320,
                ending: TrackEnding::Signal {
                    signal: SignalId::DolníMechoklatyEntry,
                    next: TrackId::DolníMechoklatyStop,
                },
            },
            TrackId::DolníMechoklatyStop => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    signal: SignalId::DolníMechoklatyExit,
                    next: TrackId::DolníMechoklatySwitch,
                },
            },
            TrackId::DolníMechoklatySwitch => TrackInfo {
                length: 20,
                ending: TrackEnding::Switch {
                    switch: SwitchId::DolníMechoklaty,
                    left: TrackId::ToPředvořanySwitchRight,
                    right: TrackId::ToKolnovRight,
                },
            },

            TrackId::ToPředvořanySwitchLeft => TrackInfo {
                length: 140,
                ending: TrackEnding::Track {
                    next: TrackId::PředvořanySwitch,
                },
            },
            TrackId::ToPředvořanySwitchRight => TrackInfo {
                length: 200,
                ending: TrackEnding::Track {
                    next: TrackId::PředvořanySwitch,
                },
            },
            TrackId::PředvořanySwitch => TrackInfo {
                length: 20,
                ending: TrackEnding::Signal {
                    signal: SignalId::PředvořanyEntry,
                    next: TrackId::PředvořanyStop,
                },
            },
            TrackId::PředvořanyStop => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    signal: SignalId::PředvořanyExit,
                    next: TrackId::ToKolnovLeft,
                },
            },

            TrackId::ToKolnovLeft => TrackInfo {
                length: 200,
                ending: TrackEnding::Signal {
                    signal: SignalId::KolnovLeftEntry,
                    next: TrackId::KolnovEntrySwitch,
                },
            },
            TrackId::ToKolnovRight => TrackInfo {
                length: 400,
                ending: TrackEnding::Signal {
                    signal: SignalId::KolnovRightEntry,
                    next: TrackId::KolnovEntryConnection,
                },
            },
            TrackId::KolnovEntrySwitch => TrackInfo {
                length: 20,
                ending: TrackEnding::Switch {
                    switch: SwitchId::KolnovEntry,
                    left: TrackId::KolnovLeftStop,
                    right: TrackId::KolnovRightStop,
                },
            },
            TrackId::KolnovEntryConnection => TrackInfo {
                length: 20,
                ending: TrackEnding::Track {
                    next: TrackId::KolnovRightStop,
                },
            },
            TrackId::KolnovLeftStop => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    signal: SignalId::KolnovLeftExit,
                    next: TrackId::KolnovExitConnection,
                },
            },
            TrackId::KolnovRightStop => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    signal: SignalId::KolnovRightExit,
                    next: TrackId::KolnovExitSwitch,
                },
            },
            TrackId::KolnovExitSwitch => TrackInfo {
                length: 20,
                ending: TrackEnding::Switch {
                    switch: SwitchId::KolnovExit,
                    left: TrackId::ToNováVesLeft,
                    right: TrackId::ToNováVesRight,
                },
            },
            TrackId::KolnovExitConnection => TrackInfo {
                length: 20,
                ending: TrackEnding::Track {
                    next: TrackId::ToNováVesLeft,
                },
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwitchId {
    NováVesLeft,
    NováVesRight,

    KolnovEntry,
    KolnovExit,

    DolníMechoklaty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignalId {
    NováVesLeftEntry,
    NováVesRightEntry,
    NováVesLeftExit,
    NováVesRightExit,

    KolnovLeftEntry,
    KolnovRightEntry,
    KolnovLeftExit,
    KolnovRightExit,

    HorníMechoklatyEntry,
    HorníMechoklatyExit,

    DolníMechoklatyEntry,
    DolníMechoklatyExit,

    PředvořanyEntry,
    PředvořanyExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrainId {
    Red,
    Blue,
}

impl TrainId {
    pub fn start(self) -> TrackId {
        match self {
            TrainId::Red => TrackId::NováVesLeftStop,
            TrainId::Blue => TrackId::NováVesRightStop,
        }
    }
}
