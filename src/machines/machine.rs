use crate::beans::bean::{Bean, BeanRoast};
use crate::beans::coffee_type::Coffee;
use chrono::Local;
use http::{Request, Response, Uri};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum IP {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IP {
    pub fn to_string(&self) -> String {
        match self {
            IP::V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
            IP::V6(s) => s.clone(),
        }
    }
}

// Simple machine with ip, and roasts
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Machine {
    //TODO: Connect to machine and prompt for info needed
    machine_ip: IP,
    port: u8,
    beans: Vec<Bean>,
    physical_location: String,
}

impl Machine {
    pub fn make_coffee(&self, total_amount: u8, coffee: Coffee) {
        //Amount = water_amount(total_amount)
        unimplemented!()
    }

    pub fn get_stats(&self) -> http::Result<Response<()>> {
        Request::builder()
            .uri(
                format!("{}:{}/", self.machine_ip.to_string(), self.port)
                    .as_str()
                    .parse::<Uri>()
                    .unwrap(),
            )
            .body(())
            .unwrap();
        unimplemented!()
    }

    pub fn client_data(&self) -> String {
        serde_json::to_string(&DummyMachine::new(
            self.physical_location.clone(),
            self.beans.clone(),
        ))
        .unwrap()
    }
    /*
    pub fn new() -> Machine {
        Machine {
            machine_ip: IP::V6("pis".to_string()),
            port: 80,
            beans: vec![Bean::new(
                BeanRoast::Dark(95),
                Local::now().date_naive(),
                "Vodskov".to_string(),
            )],
            physical_location: "Entre".to_string(),
        }
    }
     */
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct DummyMachine {
    physical_location: String,
    beans: Vec<Bean>,
}
impl DummyMachine {
    fn new(physical_location: String, beans: Vec<Bean>) -> DummyMachine {
        DummyMachine {
            physical_location,
            beans,
        }
    }
}
