use crate::beans::coffee_type::Coffee;
use crate::machines::machine::{DummyMachine, Machine};
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

    pub fn get_data(&self) -> String {
        serde_json::to_string(&Data {
            machines: self
                .machines
                .iter()
                .map(|m| m.get_dummy())
                .collect::<Vec<DummyMachine>>(),
            coffees: self.coffees.clone(),
        })
        .unwrap()
    }

    //TODO: Beans should be sent as well, currently only takes the first
    //TODO: Total amount from user
    //TODO: Should be indices for machine and coffee
    pub async fn make_coffee(
        &self,
        // m: Machine,
        // c: Coffee,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //let ma = self.machines.iter().find(|_m| **_m == m).unwrap();
        //ma.make_coffee(10, &c, &ma.beans[0]).await?;
        self.machines[0]
            .make_coffee(
                10,
                self.coffees
                    .iter()
                    .find(|c| c.name == "Black Coffee")
                    .unwrap(),
                &self.machines[0].beans[0],
            )
            .await?;
        Ok(())
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Data {
    machines: Vec<DummyMachine>,
    coffees: Vec<Coffee>,
}
