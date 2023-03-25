use chrono::Local;
use clap::{arg, value_parser, Arg, Command};
use mean_bean_machine::beans::bean::Bean;
use mean_bean_machine::beans::{bean::BeanRoast, serialization::encode_roast};
use serde_json::Result;
use std::path::PathBuf;

//TODO: args (server locations)
macro_rules! cli_args {
    () => {
        Command::new("MeanBeanMachine").version("0.1").author("t-lohse").about("Beanis")
        .arg(
            arg!(
                -c --config <FILE> "Specify the config file containing coffee-machines"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(Arg::new("coffee-machines").short('s').long("servers").num_args(0..).required(false))
        .get_matches()
    }
}

fn main() {
    let b = Bean::new(
        BeanRoast::Light(79),
        Local::now().date_naive(),
        "gamer".to_string(),
    );

    println!(
        "{:?}",
        serde_json::from_str::<Bean>(serde_json::to_string(&b).unwrap().as_str())
    );
    parse_args(cli_args!());
}

fn parse_args(args: clap::ArgMatches) {
    println!("{:?}", args);
}
