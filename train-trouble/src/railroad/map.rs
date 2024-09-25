use enum_map::Enum;
use serde::{Deserialize, Serialize};

use super::{TrackEnding, TrackInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrackId {
    MainBeginning,
    MainStop,
    MainSidingIn,
    MainSidingLeft,
    MainSidingRight,
    MainSidingOut,
    MainEnd,

    BottomTo,
    BottomStop,
    BottomBypassSwitch,
    BottomBypass,
    BottomYard,
    BottomYardFrom,

    TopTo,
    TopFactory,
    TopFrom,
}

impl TrackId {
    pub fn info(self) -> TrackInfo {
        match self {
            TrackId::MainBeginning => TrackInfo {
                length: 30,
                ending: TrackEnding::Track {
                    next: TrackId::MainStop,
                },
            },
            TrackId::MainStop => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    next: TrackId::MainSidingIn,
                    signal: SignalId::MainStop,
                },
            },
            TrackId::MainSidingIn => TrackInfo {
                length: 20,
                ending: TrackEnding::Switch {
                    switch: SwitchId::MainSiding,
                    left: TrackId::MainSidingLeft,
                    right: TrackId::MainSidingRight,
                },
            },
            TrackId::MainSidingLeft => TrackInfo {
                length: 50,
                ending: TrackEnding::Track {
                    next: TrackId::MainSidingOut,
                },
            },
            TrackId::MainSidingRight => TrackInfo {
                length: 50,
                ending: TrackEnding::Track {
                    next: TrackId::MainSidingOut,
                },
            },
            TrackId::MainSidingOut => TrackInfo {
                length: 20,
                ending: TrackEnding::Track {
                    next: TrackId::MainEnd,
                },
            },
            TrackId::MainEnd => TrackInfo {
                length: 20,
                ending: TrackEnding::Switch {
                    switch: SwitchId::MainEnd,
                    left: TrackId::BottomTo,
                    right: TrackId::TopTo,
                },
            },

            TrackId::BottomTo => TrackInfo {
                length: 200,
                ending: TrackEnding::Track {
                    next: TrackId::BottomStop,
                },
            },
            TrackId::BottomStop => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    next: TrackId::BottomBypassSwitch,
                    signal: SignalId::BottomSwitch,
                },
            },
            TrackId::BottomBypassSwitch => TrackInfo {
                length: 20,
                ending: TrackEnding::Switch {
                    switch: SwitchId::BottomBypass,
                    left: TrackId::BottomBypass,
                    right: TrackId::BottomYard,
                },
            },
            TrackId::BottomBypass => TrackInfo {
                length: 300,
                ending: TrackEnding::Track {
                    next: TrackId::MainBeginning,
                },
            },
            TrackId::BottomYard => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    signal: SignalId::BottomYard,
                    next: TrackId::BottomYardFrom,
                },
            },
            TrackId::BottomYardFrom => TrackInfo {
                length: 280,
                ending: TrackEnding::Track {
                    next: TrackId::MainBeginning,
                },
            },

            TrackId::TopTo => TrackInfo {
                length: 400,
                ending: TrackEnding::Track {
                    next: TrackId::TopFactory,
                },
            },
            TrackId::TopFactory => TrackInfo {
                length: 100,
                ending: TrackEnding::Signal {
                    signal: SignalId::TopFactory,
                    next: TrackId::TopFrom,
                },
            },
            TrackId::TopFrom => TrackInfo {
                length: 160,
                ending: TrackEnding::Track {
                    next: TrackId::MainBeginning,
                },
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwitchId {
    MainSiding,
    MainEnd,
    BottomBypass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignalId {
    MainStop,
    BottomSwitch,
    BottomYard,
    TopFactory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrainId {
    Main,
}

impl TrainId {
    pub fn start(self) -> TrackId {
        match self {
            TrainId::Main => TrackId::BottomStop,
        }
    }
}
