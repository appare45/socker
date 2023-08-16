use crate::ps::ps;

mod parse;
mod ps;

fn main() {
    let config = parse::Config::new();
    println!("{}", config.ociVersion);
    ps();
}
