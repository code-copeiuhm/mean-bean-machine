use clap::{arg, value_parser, Command};
use mean_bean_machine::backend::MeanBackend;
use mean_bean_machine::beans::coffee_type::Coffee;
use mean_bean_machine::machines::machine::Machine;
use rocket::fs::FileServer;
use std::path::PathBuf;

pub type Conf = (Vec<Machine>, Vec<Coffee>);

macro_rules! cli_args {
    () => {
        Command::new("MeanBeanMachine").version("0.1").author("t-lohse").about("Beanis")
        .arg(
            arg!(
                -c --config <FILE> "Specify the config file containing coffee-machines"
            )
            .default_value(crate::constants::DEFAULT_MACHINE)
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -t --coffees <FILE> "Specify the config file containing coffee types"
            )
            .default_value(crate::constants::DEFAULT_COFFEE)
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        //.arg(Arg::new("coffee-machines").short('s').long("servers").num_args(0..).required(false))
        .get_matches()
    }
}

#[get("/")]
fn world() -> &'static str {
    //send html side
    "Hello, world!"
}
/*
async fn test() {
    //println!("gamer2");
    //let m = mean_bean_machine::machines::machine::Machine::new();
    //println!("Machine: {:?}", m.client_data());
    //    let _ =
    //println!("p: {:?}", p);
    //let b = m.make_coffee(100).await;
    //println!("b: {:?}", b);
}
 */

#[rocket::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //m.get_stats().await?;
    let conf = crate::parse_args(cli_args!())?;
    let backend = MeanBackend::new(vec![Machine::new()], conf.1).await?;
    println!("{:?}", backend.get_data()?);

    let _rocket = rocket::build()
        .mount("/", FileServer::from("src/coffee-girl/public"))
        .launch()
        .await?;

    Ok(())
}
