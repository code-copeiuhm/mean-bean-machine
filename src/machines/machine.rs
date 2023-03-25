use std::fmt::Formatter;
use crate::beans::bean::{Bean, BeanRoast};
use crate::beans::coffee_type::Coffee;
use chrono::Local;
use http::{Request, Response, Uri};
use serde::{Deserialize, Serialize};
use IP::{V4, V6};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum IP {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl std::fmt::Display for IP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
            V6(s) => s.clone(),
        })
    }
}

// Simple machine with ip, and roasts
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Machine {
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

    pub async fn get_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get("https://httpbin.org/ip")//reqwest::get(format!("https://{}:{}/", self.machine_ip, self.port))
            .await?
            .json::<std::collections::HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok(())
        /*
        Request::builder()
            .uri(
                format!("{}:{}/", self.machine_ip.to_string(), self.port)
                    .as_str()
                    .parse::<Uri>()
                    .unwrap(),
            )
            .body(())
            .unwrap();
         */
        //unimplemented!()
    }

    pub fn client_data(&self) -> String {
        serde_json::to_string(&DummyMachine::new(
            self.physical_location.clone(),
            self.beans.clone(),
        ))
        .unwrap()
    }

    pub fn new() -> Machine {
        Machine {
            machine_ip: V4(127,0,0,1),
            port: 80,
            beans: vec![Bean::new(
                BeanRoast::Dark(95),
                Local::now().date_naive(),
                "Vodskov".to_string(),
            )],
            physical_location: "Entre".to_string(),
        }
    }

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
