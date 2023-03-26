use crate::beans::serialization::*;
use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};

pub type Temperature = u8;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BeanRoast {
    Light(Temperature),
    Medium(Temperature),
    Dark(Temperature),
}

impl BeanRoast {
    fn get_temp(&self) -> Temperature {
        match self {
            BeanRoast::Dark(t) | BeanRoast::Medium(t) | BeanRoast::Light(t) => *t,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Bean {
    roast: BeanRoast,
    #[serde(deserialize_with = "decode_date", serialize_with = "encode_date")]
    roast_date: NaiveDate,
    source: String, //TODO: Location type?
}

impl Bean {
    pub fn new(roast: BeanRoast, roast_date: NaiveDate, source: String) -> Self {
        Bean {
            roast,
            roast_date,
            source,
        }
    }

    pub fn reordered(&mut self, date: Option<NaiveDate>) {
        self.roast_date = date.unwrap_or(chrono::offset::Local::now().date_naive());
    }

    pub fn new_reordered(self, date: Option<NaiveDate>) -> Self {
        Bean {
            roast: self.roast,
            roast_date: date.unwrap_or(chrono::offset::Local::now().date_naive()),
            source: self.source,
        }
    }

    pub fn optimal_temperature(&self) -> Temperature {
        self.roast.get_temp()
    }
}
