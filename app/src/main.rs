use clap::{Parser, Subcommand};
use clone::clone;
use parse::Config;
use ps::ps;

mod clone;
mod parse;
mod ps;

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Clone,
    OciVersion,
    Ps {
        #[arg(short)]
        all: Option<bool>,
    },
}

fn main() {
    let args = CLI::parse();

    match &args.command {
        Some(Commands::OciVersion) => {
            let config = Config::new();
            println!("{}", config.oci_version);
        }
        Some(Commands::Ps { all }) => match all {
            Some(true) => ps(true),
            _ => {
                ps(false);
            }
        },
        Some(Commands::Clone) => {
            clone();
        }
        _ => {
            eprint!("Unexpected command")
        }
    }
}
