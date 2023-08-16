use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::BufReader;

// https://github.com/opencontainers/runtime-spec/blob/main/config.md

#[derive(Deserialize, Debug)]
struct Root {
    path: String,
    readonly: Option<bool>,
}

#[derive(Deserialize)]
struct Mounts {
    destination: String,
    source: Option<String>,
    options: Option<Vec<MountOption>>,
}

#[derive(Deserialize)]
enum MountOption {
    AAsync(String),
    Atime(String),
    Bind(String),
    Defaults(String),
    Dev(String),
    Diratime(String),
    Dirsync(String),
    Exec(String),
    Iversion(String),
    Lazytime(String),
    Loud(String),
    Mand(String),
    Noatime(String),
    Nodev(String),
    Nodiratime(String),
    Noexec(String),
    Noiversion(String),
    Nolazytime(String),
    Nomand(String),
    Norelatime(String),
    Nostrictatime(String),
    Nosuid(String),
    Nosymfollow(String),
    Private(String),
    Ratime(String),
    Rbind(String),
    Rdev(String),
    Rdiratime(String),
    Relatime(String),
    Remount(String),
    Rexec(String),
    Rnoatime(String),
    Rnodiratime(String),
    Rnoexec(String),
    Rnorelatime(String),
    Rnostrictatime(String),
    Rnosuid(String),
    Rnosymfollow(String),
    Ro(String),
    Rprivate(String),
    Rrelatime(String),
    Rro(String),
    Rrw(String),
    Rshared(String),
    Rslave(String),
    Rstrictatime(String),
    Rsuid(String),
    Rsymfollow(String),
    Runbindable(String),
    Rw(String),
    Shared(String),
    Silent(String),
    Slave(String),
    Strictatime(String),
    Suid(String),
    Symfollow(String),
    Sync(String),
    Tmpcopyup(String),
    Unbindable(String),
}

#[derive(Deserialize)]
struct ConsoleSize {
    height: u32,
    width: u32,
}
#[derive(Deserialize)]
struct RLimits {
    r_type: String,
    r_soft: u64,
    r_hard: u64,
}
#[derive(Deserialize)]
struct Process {
    terminal: Option<bool>,
    console_size: Option<ConsoleSize>,
    cwd: String,
    env: Option<Vec<String>>,
    args: Option<Vec<String>>,
    command_line: Option<String>,
    rlimits: Option<RLimits>, // Only supported on POSIX
}

#[derive(Deserialize)]
pub struct Config {
    pub oci_version: String,
    root: Option<Root>,
}

impl Config {
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
    fn parse_config_file(file: &File) -> Config {
        let reader = BufReader::new(file);
        let c = match serde_json::from_reader(reader) {
            Ok(config) => config,
            Err(e) => panic!("Error desirializeing config.json: {:?}", e),
        };
        return c;
    }

    pub fn new() -> Self {
        let c = Self::read_config_file();
        let config = Self::parse_config_file(&c);
        return config;
    }
}
