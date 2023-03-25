pub mod backend;
pub mod beans;
pub mod machines;

pub use beans::supplier::*;

pub mod constants {
    pub const DEFAULT_COFFEE: &str = "./coffees.json";
    pub const DEFAULT_MACHINE: &str = "./machines.json";
}
