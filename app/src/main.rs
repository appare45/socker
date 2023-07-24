use std::fs::File;

fn main() {
    let c =read_config_file();
    print!("{:?}", c);
}

fn read_config_file() -> File {
    let f = match File::open("config.json") {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {:?}", e)
    };
    return f;
}