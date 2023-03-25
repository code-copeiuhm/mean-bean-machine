use crate::beans::bean::{Bean, BeanRoast};
use chrono::Local;
use http::uri::InvalidUri;
use hyper::{body::HttpBody, Client};
use rocket::http::hyper as rhyper;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use IP::{V4, V6};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum IP {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl std::fmt::Display for IP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
                V6(s) => s.clone(),
            }
        )
    }
}

// Simple machine with ip, and roasts
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Machine {
    machine_ip: IP,
    port: u16,
    beans: Vec<Bean>,
    physical_location: String,
    pots: u8,
}

impl Machine {
    fn get_addr(&self, add: Option<&str>) -> Result<rhyper::Uri, InvalidUri> {
        format!(
            "http://{}:{}/{}",
            self.machine_ip,
            self.port,
            add.unwrap_or("")
        )
        .parse()
    }

    pub async fn make_coffee(
        //TODO: write
        &self,
        total_amount: u8,
        //coffee: &Coffee,
        //bean: &Bean
    ) -> Result<(), Box<dyn std::error::Error>> {
        let po = Query::new(0, 10, 100, 30);
        println!("{}", serde_json::to_string(&po)?);
        let req = rhyper::Request::builder()
            .method(rhyper::Method::POST)
            .uri(self.get_addr(Some("pot-0"))?)
            .header("content-type", "application/coffee-pot-command")
            .header("content-type", "application/coffee-pot-command")
            .body(rhyper::Body::from(
                //serde_json::to_string(&po)?.as_bytes()
                r#"{"canister": 0, "beanAmount": 10, "temp": 100, "mil":30}"#,
            ))?;
        println!("{:?}", req.body());
        let client = Client::new();

        // POST it...
        let mut resp = client.request(req).await?;

        println!("Response: {}", resp.status());
        let s = String::from_utf8(resp.body_mut().data().await.unwrap().unwrap().to_vec())?;
        println!("s: {}", s.as_str());
        //  println!("s: {:?}", serde_json::from_str::<S>(s.as_str()));
        //Amount = water_amount(total_amount)
        Ok(())
    }

    pub async fn get_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        // Parse an `http::Uri`...
        let uri = self.get_addr(Some("propfind"))?;

        // Await the response...
        let mut resp = client.get(uri).await?;

        //println!("Response: {:#?}", resp.status());
        // And now...
        //let s = String::from_utf8(resp.body_mut().data().await.unwrap().unwrap().to_vec())?;
        let s = serde_json::from_str::<Stats>(
            String::from_utf8(resp.body_mut().data().await.unwrap().unwrap().to_vec())?.as_str(),
        )?;
        //println!("s: {:?}", s);
        assert_eq!(s.opots.len() as u8, self.pots);
        assert_eq!(s.canisters.len(), self.beans.len());

        Ok(())
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
            machine_ip: V4(127, 0, 0, 1),
            port: 5000,
            beans: vec![
                Bean::new(
                    BeanRoast::Dark(95),
                    Local::now().date_naive(),
                    "Vodskov".to_string(),
                ),
                Bean::new(
                    BeanRoast::Light(91),
                    Local::now().date_naive(),
                    "Vodskov".to_string(),
                ),
                Bean::new(
                    BeanRoast::Medium(93),
                    Local::now().date_naive(),
                    "Vodskov".to_string(),
                ),
            ],
            physical_location: "Entre".to_string(),
            pots: 1,
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

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Query {
    canister: u8,
    #[serde(rename = "beenAmount")]
    grams: u8,
    temp: u8,
    #[serde(rename = "mil")]
    water: u8,
}
impl Query {
    fn new(canister: u8, grams: u8, temp: u8, water: u8) -> Query {
        Query {
            canister,
            grams,
            temp,
            water,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Stats {
    #[serde(rename = "accepted-additions")]
    accepted: Vec<String>,
    canisters: Vec<u32>,
    logs: Vec<String>,
    opots: Vec<u8>,
}
