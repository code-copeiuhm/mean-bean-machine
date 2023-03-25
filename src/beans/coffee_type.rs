use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MilkType {
    Cream,
    #[serde(rename = "Half-and-half")]
    HalfAndHalf,
    #[serde(rename = "Whole-milk")]
    Whole,
    #[serde(rename = "Part-Skim")]
    PartSkim,
    Skim,
    #[serde(rename = "Non-Dairy")]
    NonDairy,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Coffee {
    name: String,
    water_ratio: u8,
    milk_ratio: Option<u8>,
    milk_type: Option<MilkType>,
}

impl Coffee {
    pub fn new(
        name: String,
        water_ratio: u8,
        milk_ratio: Option<u8>,
        milk_type: Option<MilkType>,
    ) -> Coffee {
        assert_eq!(milk_ratio.is_some(), milk_type.is_some());
        Coffee {
            name,
            water_ratio,
            milk_ratio,
            milk_type,
        }
    }
    fn water_ratio(&self) -> u8 {
        self.water_ratio
    }
    fn milk_type(&self) -> &Option<MilkType> {
        &self.milk_type
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
