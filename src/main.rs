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
    match server::main() {
        Ok(c) => c,
        Err(e) => {
            println!("Argument parse error: {e}");
            std::process::exit(69);
        }
    };
}

fn parse_args(args: clap::ArgMatches) -> Result<server::Conf, Box<dyn Error>> {
    // Servers/Coffee machines
    let mut s = String::new();

    //TODO: Fill file
    File::open(args.get_one::<PathBuf>("config").unwrap())?.read_to_string(&mut s)?;
    let machines = serde_json::from_str::<Vec<Machine>>(s.as_str())?;

    // Coffee
    File::open(args.get_one::<PathBuf>("coffees").unwrap())?.read_to_string(&mut s)?;
    let coffees = serde_json::from_str::<Vec<Coffee>>(s.as_str())?;

    Ok((machines, coffees))
}
