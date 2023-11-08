use clap::{Parser, Subcommand};
use clone::new_uts;
use mount::mount_bind;
use parse::Config;
use ps::ps;

mod clone;
mod mount;
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
    Mount {
        src: String,
        to: String,
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
            new_uts();
        }
        Some(Commands::Mount { src, to }) => mount_bind(src, to),
        _ => {
            eprint!("Unexpected command")
        }
    }
}
