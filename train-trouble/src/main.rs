use serde::{Deserialize, Serialize};
use train_trouble_engine::{ActionResult, Game};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SampleGame {
    value: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Channel {
    View,
    Change,
    Reset,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum View {
    Value { value: u64 },
    Change,
    Reset,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Action {
    Increment,
    Decrement,
    Reset,
}

impl Game for SampleGame {
    type CHANNEL = Channel;
    type VIEW = View;
    type ACTION = Action;

    const TICK_RATE: u64 = 20;

    fn tick(&mut self) {
        self.value += 1;
    }

    fn view(&mut self, channel: Self::CHANNEL) -> Self::VIEW {
        match channel {
            Channel::View => View::Value {
                value: self.value / Self::TICK_RATE,
            },
            Channel::Change => View::Change,
            Channel::Reset => View::Reset,
        }
    }

    fn act(&mut self, channel: Self::CHANNEL, action: Self::ACTION) -> ActionResult {
        match (channel, action) {
            (Channel::Change, Action::Increment) => {
                self.value = self.value.saturating_add(10 * Self::TICK_RATE);
                ActionResult::Ok
            }
            (Channel::Change, Action::Decrement) => {
                self.value = self.value.saturating_sub(10 * Self::TICK_RATE);
                ActionResult::Ok
            }
            (Channel::Reset, Action::Reset) => {
                self.value = 0;
                ActionResult::Ok
            }
            _ => ActionResult::Misdirected,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
