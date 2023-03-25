use crate::beans::coffee_type::Coffee;
use crate::machines::machine::Machine;

pub struct MeanBackend {
    machines: Vec<Machine>,
    coffees: Vec<Coffee>,
}

impl MeanBackend {
    pub fn new(machines: Vec<Machine>, coffees: Vec<Coffee>) -> MeanBackend {
        machines.iter().for_each(|m| m.get_stats());
        MeanBackend { machines, coffees }
    }
}
