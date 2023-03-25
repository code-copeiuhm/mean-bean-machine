use clap::{arg, value_parser, Arg, Command};
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
    parse_args(cli_args!());
}

fn parse_args(args: clap::ArgMatches) {
    println!("{:?}", args);
}
