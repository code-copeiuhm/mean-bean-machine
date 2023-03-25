use chrono::Local;
use clap::{arg, value_parser, Arg, Command};
use mean_bean_machine::beans::bean::Bean;
use mean_bean_machine::beans::bean::BeanRoast;
use mean_bean_machine::beans::coffee_type::Coffee;
use mean_bean_machine::machines::machine::Machine;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

static DEFAULT_COFFEE: &str = "./coffees.json";
static DEFAULT_MACHINE: &str = "./machines.json";

//TODO: args (server locations)
macro_rules! cli_args {
    () => {
        Command::new("MeanBeanMachine").version("0.1").author("t-lohse").about("Beanis")
        .arg(
            arg!(
                -c --config <FILE> "Specify the config file containing coffee-machines"
            )
            .default_value(DEFAULT_MACHINE)
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -t --coffees <FILE> "Specify the config file containing coffee types"
            )
            .default_value(DEFAULT_COFFEE)
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        //.arg(Arg::new("coffee-machines").short('s').long("servers").num_args(0..).required(false))
        .get_matches()
    }
}

fn main() {
    parse_args(cli_args!());
}

fn parse_args(args: clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    // Servers/Coffee machines
    let mut s = String::new();
    File::open(args.get_one::<PathBuf>("config").unwrap())?.read_to_string(&mut s)?;
    let machines = serde_json::from_str::<Vec<Machine>>(s.as_str())?;

    // Coffee
    File::open(args.get_one::<PathBuf>("coffees").unwrap())?.read_to_string(&mut s)?;
    let coffees = serde_json::from_str::<Vec<Coffee>>(s.as_str())?;

    Ok(())
}
