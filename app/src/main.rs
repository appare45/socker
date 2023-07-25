use std::fs::File;

fn main() {
    let c =read_config_file();
    print!("{:?}", c);
}

fn read_config_file() -> File {
    let file = match File::open("config.json") {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {:?}", e)
    };

    // validate filetype
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => panic!("Error reading metadata: {:?}", e)
    };
    if !metadata.is_file() {
        panic!("Error: config.json is not a file");
    }
    if metadata.len() == 0 {
        panic!("Error: config.json is empty");
    }

    return file;
}