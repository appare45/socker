use clap::{Parser, Subcommand};
use parse::Config;

mod parse;
mod ps;

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    OciVersion,
    Ps,
}

fn main() {
    let args = CLI::parse();

    match &args.command {
        Some(Commands::OciVersion) => {
            let config = Config::new();
            println!("{}", config.oci_version);
        }
        Some(Commands::Ps) => {
            ps::ps();
        }
        _ => {}
    }
}
