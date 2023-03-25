use crate::beans::bean::BeanRoast;
use crate::beans::coffee_type::Coffee;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum IP {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Simple machine with ip, and roasts
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Machine {
    //TODO: Connect to machine and prompt for info needed
    machine_ip: IP,
    roasts: Vec<BeanRoast>,
}

impl Machine {
    fn make_coffee(&self, total_amount: u8, coffee: Coffee) {
        //Amount = water_amount(total_amount)
    }
}
