use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Coffee {
    name: String,
    water_ratio: u8,
    milk_ratio: Option<u8>,
}

impl Coffee {
    pub fn new(name: String, water_ratio: u8, milk_ratio: Option<u8>) -> Coffee {
        Coffee {
            name,
            water_ratio,
            milk_ratio,
        }
    }
    fn water_ratio(&self) -> u8 {
        self.water_ratio
    }
    fn milk_ratio(&self) -> Option<u8> {
        self.milk_ratio
    }
    pub fn water_amount(&self, total_amount: u8) -> u8 {
        total_amount / (self.water_ratio - self.milk_ratio.unwrap_or_default())
    }
    pub fn milk_amount(&self, total_amount: u8) -> Option<u8> {
        Some(total_amount / (self.milk_ratio? - self.water_ratio))
    }
    pub fn contains_milk(&self) -> bool {
        self.milk_ratio.is_some()
    }
}
