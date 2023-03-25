use serde::{Deserialize, Serialize};
//use crate::beans::serialization::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Coffee {
    name: String,
    water_ratio: u8,
    milk_ratio: Option<u8>,
}

impl Coffee {
    fn water_ratio(&self) -> u8 {
        self.water_ratio
    }
    fn milk_ratio(&self) -> Option<u8> {
        self.milk_ratio
    }
    fn water_amount(&self, total_amount: u8) -> u8 {
        total_amount / (self.water_ratio - self.milk_ratio.unwrap_or_default())
    }
    fn contains_milk(&self) -> bool {
        self.milk_ratio.is_some()
    }
}