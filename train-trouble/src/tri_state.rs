use std::ops::Not;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct TriStateController<T> {
    target: T,
    counter: u8,
}

impl<T> TriStateController<T>
where
    T: Eq + Not<Output = T> + Copy,
{
    const DELAY: u8 = 30;

    pub fn set(&mut self, value: T) {
        if value != self.target {
            self.target = value;
            self.counter = Self::DELAY;
        }
    }

    pub fn tick(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn state(&self) -> T {
        if self.counter == 0 {
            self.target
        } else {
            !self.target
        }
    }

    pub fn tri_state(&self) -> Option<T> {
        if self.counter == 0 {
            Some(self.target)
        } else {
            None
        }
    }
}
