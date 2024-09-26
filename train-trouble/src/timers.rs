use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Interval<const INTERVAL: u64> {
    value: u64,
}

impl<const INTERVAL: u64> Interval<INTERVAL> {
    pub fn tick(&mut self) -> bool {
        if self.value == 0 {
            self.value = INTERVAL - 1;
            true
        } else {
            self.value -= 1;
            false
        }
    }
}

impl<const INTERVAL: u64> Default for Interval<INTERVAL> {
    fn default() -> Self {
        Self {
            value: INTERVAL - 1,
        }
    }
}
