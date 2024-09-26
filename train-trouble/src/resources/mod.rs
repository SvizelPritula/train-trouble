use enum_map::Enum;
use serde::{Deserialize, Serialize};

pub use market::Market;

mod market;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Resource {
    Wood,
    Wheat,
    Coal,
    Iron,
}
