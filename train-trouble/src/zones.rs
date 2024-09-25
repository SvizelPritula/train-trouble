use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::railroad::{Direction, RailwayState, SignalId, SwitchId};

pub struct ZoneInfo {
    pub switches: &'static [SwitchId],
    pub signals: &'static [SignalId],
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ZoneId {
    Main,
    Bottom,
    Top,
}

impl ZoneId {
    pub fn info(self) -> ZoneInfo {
        match self {
            ZoneId::Main => ZoneInfo {
                switches: &[SwitchId::MainSiding, SwitchId::MainEnd],
                signals: &[SignalId::MainStop],
            },
            ZoneId::Bottom => ZoneInfo {
                switches: &[SwitchId::BottomBypass],
                signals: &[SignalId::BottomYard, SignalId::BottomSwitch],
            },
            ZoneId::Top => ZoneInfo {
                switches: &[],
                signals: &[SignalId::TopFactory],
            },
        }
    }

    pub fn switches(self, railway: &RailwayState) -> Vec<SwitchView> {
        self.info()
            .switches
            .iter()
            .copied()
            .map(|id| SwitchView {
                id,
                direction: railway.switches[id].stable_direction(),
            })
            .collect()
    }

    pub fn signals(self, railway: &RailwayState) -> Vec<SignalView> {
        self.info()
            .signals
            .iter()
            .copied()
            .map(|id| SignalView {
                id,
                clear: railway.signals[id].is_clear.tri_state(),
            })
            .collect()
    }
}
