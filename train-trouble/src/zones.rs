use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::{
    railroad::{SignalId, SwitchId, TrainId},
    TrainToubleGame,
};

pub struct ZoneInfo {
    pub neighbours: &'static [ZoneId],
    pub switches: &'static [SwitchId],
    pub signals: &'static [SignalId],
    pub platforms: &'static [SignalId],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ZoneId {
    NováVes,
    Kolnov,
    HorníMechoklaty,
    DolníMechoklaty,
    Předvořany,
}

impl ZoneId {
    pub fn info(self) -> ZoneInfo {
        match self {
            ZoneId::NováVes => ZoneInfo {
                neighbours: &[
                    ZoneId::HorníMechoklaty,
                    ZoneId::DolníMechoklaty,
                    ZoneId::Kolnov,
                ],
                switches: &[SwitchId::NováVesLeft, SwitchId::NováVesRight],
                signals: &[
                    SignalId::NováVesLeftEntry,
                    SignalId::NováVesRightEntry,
                    SignalId::NováVesLeftExit,
                    SignalId::NováVesRightExit,
                ],
                platforms: &[SignalId::NováVesLeftExit, SignalId::NováVesRightExit],
            },
            ZoneId::Kolnov => ZoneInfo {
                neighbours: &[ZoneId::NováVes, ZoneId::Předvořany],
                switches: &[SwitchId::KolnovEntry, SwitchId::KolnovExit],
                signals: &[
                    SignalId::KolnovLeftEntry,
                    SignalId::KolnovRightEntry,
                    SignalId::KolnovLeftExit,
                    SignalId::KolnovRightExit,
                ],
                platforms: &[SignalId::KolnovLeftExit, SignalId::KolnovRightExit],
            },
            ZoneId::HorníMechoklaty => ZoneInfo {
                neighbours: &[ZoneId::NováVes, ZoneId::Předvořany],
                switches: &[],
                signals: &[
                    SignalId::HorníMechoklatyEntry,
                    SignalId::HorníMechoklatyExit,
                ],
                platforms: &[SignalId::HorníMechoklatyExit],
            },
            ZoneId::DolníMechoklaty => ZoneInfo {
                neighbours: &[ZoneId::NováVes, ZoneId::Předvořany],
                switches: &[SwitchId::DolníMechoklaty],
                signals: &[
                    SignalId::DolníMechoklatyEntry,
                    SignalId::DolníMechoklatyExit,
                ],
                platforms: &[SignalId::DolníMechoklatyExit],
            },
            ZoneId::Předvořany => ZoneInfo {
                neighbours: &[
                    ZoneId::Kolnov,
                    ZoneId::HorníMechoklaty,
                    ZoneId::DolníMechoklaty,
                ],
                switches: &[],
                signals: &[SignalId::PředvořanyEntry, SignalId::PředvořanyExit],
                platforms: &[SignalId::PředvořanyExit],
            },
        }
    }
}

impl TrainToubleGame {
    pub fn train_in_zone(&self, train: TrainId, zone: ZoneId) -> bool {
        zone.info()
            .platforms
            .iter()
            .flat_map(|p| self.railway.trains_at_signal(*p))
            .any(|(t, stopped)| t == train && stopped)
    }
}
