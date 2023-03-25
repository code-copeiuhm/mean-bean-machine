use mean_bean_machine::beans::coffee_type::Coffee;
use mean_bean_machine::machines::machine::Machine;

use mean_bean_machine::constants;
use std::error::Error;
use std::fs::File;
use std::future::Future;
use std::io::Read;
use std::path::PathBuf;

#[macro_use]
extern crate rocket;

mod server;

fn main() {
    match server::main() {
        Ok(c) => c,
        Err(e) => {
            println!("Argument parse error: {e}");
            std::process::exit(69);
        }
    }
}

fn parse_args(args: clap::ArgMatches) -> Result<server::Conf, Box<dyn Error>> {
    // Servers/Coffee machines
    let mut s = String::new();

    //TODO: Fill file
    //File::open(args.get_one::<PathBuf>("config").unwrap())?.read_to_string(&mut s)?;
    //let machines = serde_json::from_str::<Vec<Machine>>(s.as_str())?;
    let machines = vec![];//serde_json::from_str::<Vec<Machine>>(s.as_str())?;

    // Coffee
    File::open(args.get_one::<PathBuf>("coffees").unwrap())?.read_to_string(&mut s)?;
    let coffees = serde_json::from_str::<Vec<Coffee>>(s.as_str())?;

    Ok((machines, coffees))
}
