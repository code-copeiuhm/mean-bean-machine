use crate::beans::coffee_type::Coffee;
use crate::machines::machine::Machine;
use http::{Request, Response};

pub struct MeanBackend {
    machines: Vec<Machine>,
    coffees: Vec<Coffee>,
}

impl MeanBackend {
    pub fn new(machines: Vec<Machine>, coffees: Vec<Coffee>) -> MeanBackend {
        machines.iter().for_each(|m| {
            let _ = m.get_stats();
        });
        MeanBackend { machines, coffees }
    }
}
