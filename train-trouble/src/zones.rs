use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::railroad::{SignalId, SwitchId};

pub struct ZoneInfo {
    pub switches: &'static [SwitchId],
    pub signals: &'static [SignalId],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize)]
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
}
