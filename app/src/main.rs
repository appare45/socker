use anyhow::{ensure, Result};
use clap::Parser;

use socker::{config_parser::Config, container::Container};
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author="appare45")]
struct Args {
    /// The target of config file
    name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config_path = Path::new(&args.name);
    // Ensure the config file exists
    ensure!(
        config_path.is_file(),
        "{} is not a file",
        config_path.display()
    );
    let config_data = std::fs::read_to_string(config_path)?;
    let config = Config::try_from(config_data.as_str())?;
    let mut container = Container::new(config);
    container.run()?;
    Ok(())
}
