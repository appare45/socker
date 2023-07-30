use std::fs::File;
use std::io::BufReader;
use serde_json;
use serde::Deserialize;

fn main() {
    let c = read_config_file();
    let config = parse_config_file(&c);
    println!("{}", config.ociVersion);
}

fn read_config_file() -> File {
    let file = match File::open("config.json") {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {:?}", e),
    };

    // validate filetype
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => panic!("Error reading metadata: {:?}", e),
    };

    if !metadata.is_file() {
        panic!("Error: config.json is not a file");
    }

    if metadata.len() == 0 {
        panic!("Error: config.json is empty");
    }

    return file;
}

// https://github.com/opencontainers/runtime-spec/blob/main/config.md
#[derive(Deserialize)]
struct Config {
    ociVersion: String,
}

fn parse_config_file(file: &File) -> Config {
    let reader = BufReader::new(file);
    let c = match serde_json::from_reader(reader) {
        Ok(config) => config,
        Err(e) => panic!("Error desirializeing config.json: {:?}", e)
    };
    return c;
}