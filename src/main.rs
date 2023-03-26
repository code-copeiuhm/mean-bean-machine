use mean_bean_machine::beans::coffee_type::Coffee;
use mean_bean_machine::machines::machine::Machine;

use mean_bean_machine::constants;
use std::error::Error;
use std::fs::File;
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
    let (mut m, mut c) = (String::new(), String::new());

    File::open(args.get_one::<PathBuf>("config").unwrap())?.read_to_string(&mut m)?;
    let machines = serde_json::from_str::<Vec<Machine>>(m.as_str())?;

    // Coffee
    File::open(args.get_one::<PathBuf>("coffees").unwrap())?.read_to_string(&mut c)?;
    let coffees = serde_json::from_str::<Vec<Coffee>>(c.as_str())?;

    Ok((machines, coffees))
}
