use crate::beans::bean::Bean;
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
    port: u8,
    beans: Vec<Bean>,
}

impl Machine {
    pub fn make_coffee(&self, total_amount: u8, coffee: Coffee) {
        //Amount = water_amount(total_amount)
        unimplemented!()
    }

    pub fn get_stats(&self) {}
}
