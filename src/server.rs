use clap::{arg, value_parser, Command};
use mean_bean_machine::backend::MeanBackend;
use mean_bean_machine::beans::coffee_type::Coffee;
use mean_bean_machine::machines::machine::Machine;
use rocket::fs::FileServer;
use rocket::State;
use std::path::PathBuf;
use std::sync::Arc;

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

#[get("/data")]
fn data(state: &State<Arc<MeanBackend>>) -> String {
    state.get_data()
}

#[get("/brew")]
async fn brew(state: &State<Arc<MeanBackend>>) -> String {
    //println!("{}, {}", coffee_machines, coffees);

    //let m: Machine = serde_json::from_str(coffee_machines.as_str()).unwrap();
    //let c: Coffee = serde_json::from_str(coffees.as_str()).unwrap();
    println!("Making coffee");
    let _ = state.make_coffee().await;
    String::from("Coffee was made")
}

#[rocket::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = crate::parse_args(cli_args!())?;
    let backend = MeanBackend::new(conf.0, conf.1).await?;

    let _ = rocket::build()
        .manage(Arc::new(backend))
        .mount("/", FileServer::from("src/coffee-girl/public"))
        .mount("/", routes![data])
        .mount("/", routes![brew])
        .launch()
        .await?;

    Ok(())
}
