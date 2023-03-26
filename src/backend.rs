use crate::beans::coffee_type::Coffee;
use crate::machines::machine::{DummyMachine, Machine};
use rocket::data::Outcome;
use rocket::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MeanBackend {
    machines: Vec<Machine>,
    coffees: Vec<Coffee>,
}

impl MeanBackend {
    pub async fn new(
        machines: Vec<Machine>,
        coffees: Vec<Coffee>,
    ) -> Result<MeanBackend, Box<dyn std::error::Error>> {
        machines.iter().for_each(|m| {
            let _ = async { m.get_stats().await };
        });
        Ok(MeanBackend { machines, coffees })
    }

    pub fn get_data(&self) -> serde_json::Result<String> {
        serde_json::to_string(&Data {
            machines: self
                .machines
                .iter()
                .map(|m| m.get_dummy())
                .collect::<Vec<DummyMachine>>(),
            coffees: self.coffees.clone(),
        })
    }

    //TODO: Machine
    pub async fn make_coffee(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Data {
    machines: Vec<DummyMachine>,
    coffees: Vec<Coffee>,
}
