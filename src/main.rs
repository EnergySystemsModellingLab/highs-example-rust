//! Provides the main entry point to the highs_example_rust program.

mod constraint;
mod optimisation;
mod settings;
mod solver;
mod variable_definition;

#[cfg(test)]
mod test_common;

use std::env;
use std::path::Path;

/// The main entry point to the program
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must provide path to model configuration TOML file.");
    }

    optimisation::run(Path::new(&args[1]))
}
