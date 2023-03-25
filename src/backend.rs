use crate::beans::coffee_type::Coffee;
use crate::machines::machine::Machine;

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
}
