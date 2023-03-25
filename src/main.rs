use clap::{arg, value_parser, Command};

use mean_bean_machine::beans::coffee_type::Coffee;
use mean_bean_machine::machines::machine::Machine;

use mean_bean_machine::constants;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
#[macro_use] extern crate rocket;

mod server;

macro_rules! cli_args {
    () => {
        Command::new("MeanBeanMachine").version("0.1").author("t-lohse").about("Beanis")
        .arg(
            arg!(
                -c --config <FILE> "Specify the config file containing coffee-machines"
            )
            .default_value(constants::DEFAULT_MACHINE)
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -t --coffees <FILE> "Specify the config file containing coffee types"
            )
            .default_value(constants::DEFAULT_COFFEE)
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        //.arg(Arg::new("coffee-machines").short('s').long("servers").num_args(0..).required(false))
        .get_matches()
    }
}

fn main() {
    //let m = Machine::new();
    //println!("Machine: {:?}", m.client_data());

    /* Test shit
    let c = vec![
        Coffee::new("Black Coffee".to_string(), 15, None, None),
        Coffee::new("Cafe Latte".to_string(), 2, Some(5), Some(MilkType::Skim)),
        Coffee::new("Espresso".to_string(), 2, None, None),
    ];
    println!("{:?}", serde_json::to_string(&c).unwrap().as_str());
    let b = Bean::new(
        BeanRoast::Light(79),
        Local::now().date_naive(),
        "gamer".to_string(),
    );

    println!(
        "{:?}",
        serde_json::from_str::<Bean>(serde_json::to_string(&b).unwrap().as_str())
    );
     */
    match parse_args(cli_args!()) {
        Ok(_) => { }
        Err(e) => {
            println!("Argument parse error: {e}");
            std::process::exit(69);
        },
    };
    server::main();
}

fn parse_args(args: clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    // Servers/Coffee machines
    let mut s = String::new();

    //TODO: Fill file
    //File::open(args.get_one::<PathBuf>("config").unwrap())?.read_to_string(&mut s)?;
    //let machines = serde_json::from_str::<Vec<Machine>>(s.as_str())?;

    // Coffee
    File::open(args.get_one::<PathBuf>("coffees").unwrap())?.read_to_string(&mut s)?;
    let coffees = serde_json::from_str::<Vec<Coffee>>(s.as_str())?;

    Ok(())
}
