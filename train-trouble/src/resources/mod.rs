use enum_map::Enum;
use serde::{Deserialize, Serialize};

pub use market::Market;

use crate::{railroad::TrainId, zones::ZoneId, TrainToubleGame};

mod market;

const MAX_TRAIN_LOAD: u64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Resource {
    Wood,
    Coal,
    Iron,
    Wool,
    Wheat,
    Salt,
}

impl Resource {
    pub fn name(self) -> &'static str {
        match self {
            Resource::Wood => "Dřevo",
            Resource::Coal => "Uhlí",
            Resource::Iron => "Železo",
            Resource::Wool => "Vlna",
            Resource::Wheat => "Obilí",
            Resource::Salt => "Sůl",
        }
    }
}

impl TrainToubleGame {
    pub fn train_load(&self, train: TrainId) -> u64 {
        self.loads[train].values().sum()
    }

    pub fn buy(&mut self, zone: ZoneId, train: TrainId, resource: Resource, amount: u64) -> bool {
        if self.train_load(train) + amount > MAX_TRAIN_LOAD {
            return false;
        }

        let price = self.market.rates[zone][resource] * amount;

        self.loads[train][resource] += amount;
        self.balance -= price as i64;

        true
    }

    pub fn sell(&mut self, zone: ZoneId, train: TrainId, resource: Resource, amount: u64) -> bool {
        if self.loads[train][resource] < amount {
            return false;
        }

        let price = self.market.rates[zone][resource] * amount;

        self.loads[train][resource] -= amount;
        self.balance += price as i64;

        true
    }
}
