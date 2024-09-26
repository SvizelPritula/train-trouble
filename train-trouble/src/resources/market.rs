use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use enum_map::EnumMap;
use rand::{seq::SliceRandom, Rng as _, SeedableRng};
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use crate::{timers::Interval, zones::ZoneId};

use super::Resource;

const MIN_RATE: u64 = 100;
const MAX_RATE: u64 = 1000;

const DEFAULT_RATE: u64 = 500;

const SEED: u64 = 0x4814ae3d;

const UPDATE_INTERVAL: u64 = 15 * 20;
const UPDATES_PER_EVENT: RangeInclusive<u64> = 6..=10;

const AVERAGING_PULL_DENOMINATOR: i64 = 12;
const MAX_CHANGE_DENOMINATOR: u64 = 4;
const MIN_CHANGE_DENOMINATOR: u64 = 6;
const NOISE_RANGE: RangeInclusive<i64> = -2..=2;

type Rng = Pcg64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub rates: EnumMap<ZoneId, EnumMap<Resource, u64>>,
    rng: Rng,
    update_interval: Interval<UPDATE_INTERVAL>,
    updates_until_event: u64,
}

impl Market {
    pub fn tick(&mut self) {
        if self.update_interval.tick() {
            self.average_neighbours();
            self.add_noise();

            if self.updates_until_event == 0 {
                self.updates_until_event = self.rng.gen_range(UPDATES_PER_EVENT);
                self.generate_event();
            } else {
                self.updates_until_event -= 1;
            }
        }
    }

    fn average_neighbours(&mut self) {
        let mut new_market = self.rates;

        for (zone, rates) in self.rates {
            let info = zone.info();
            let neighbours = info.neighbours.len() as i64;

            if info.neighbours.is_empty() {
                continue;
            }

            let mut deltas: EnumMap<Resource, i64> = EnumMap::default();

            for neigbour in info.neighbours {
                for (resource, rate) in self.rates[*neigbour] {
                    let delta = rate as i64 - rates[resource] as i64;
                    deltas[resource] += delta;
                }
            }

            let mut rates = rates;

            for (resource, delta) in deltas {
                rates[resource] = rates[resource]
                    .saturating_add_signed(delta / (AVERAGING_PULL_DENOMINATOR * neighbours));
            }

            new_market[zone] = rates;
        }

        self.rates = new_market;
    }

    fn add_noise(&mut self) {
        for rates in self.rates.values_mut() {
            for rate in rates.values_mut() {
                let delta = self.rng.gen_range(NOISE_RANGE);

                *rate = min(rate.saturating_add_signed(delta), MAX_RATE);
                *rate = min(*rate, MAX_RATE);
                *rate = max(*rate, MIN_RATE);
            }
        }
    }

    fn generate_event(&mut self) {
        let rates = self.rates.as_mut_array().choose_mut(&mut self.rng).unwrap();
        let rate = rates.as_mut_array().choose_mut(&mut self.rng).unwrap();

        let max_change = *rate / MAX_CHANGE_DENOMINATOR;
        let min_change = *rate / MIN_CHANGE_DENOMINATOR;
        let change = self.rng.gen_range(min_change..=max_change);

        let dec_rate = *rate - change;
        let inc_rate = *rate + change;

        *rate = if dec_rate < MIN_RATE {
            inc_rate
        } else if inc_rate > MAX_RATE {
            dec_rate
        } else {
            if self.rng.gen() {
                inc_rate
            } else {
                dec_rate
            }
        }
    }
}

impl Default for Market {
    fn default() -> Self {
        Self {
            rates: EnumMap::from_fn(|_| EnumMap::from_fn(|_| DEFAULT_RATE)),
            rng: Rng::seed_from_u64(SEED),
            update_interval: Interval::default(),
            updates_until_event: *UPDATES_PER_EVENT.start(),
        }
    }
}
