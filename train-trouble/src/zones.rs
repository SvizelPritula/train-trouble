use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::railroad::{SignalId, SwitchId};

pub struct ZoneInfo {
    pub switches: &'static [SwitchId],
    pub signals: &'static [SignalId],
    pub platforms: &'static [SignalId],
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
                signals: &[
                    SignalId::MainStop,
                    SignalId::MainSidingLeft,
                    SignalId::MainSidingRight,
                ],
                platforms: &[SignalId::MainSidingLeft, SignalId::MainSidingRight],
            },
            ZoneId::Bottom => ZoneInfo {
                switches: &[SwitchId::BottomBypass],
                signals: &[SignalId::BottomYard, SignalId::BottomSwitch],
                platforms: &[SignalId::BottomYard],
            },
            ZoneId::Top => ZoneInfo {
                switches: &[],
                signals: &[SignalId::TopFactory],
                platforms: &[SignalId::TopFactory],
            },
        }
    }
}
