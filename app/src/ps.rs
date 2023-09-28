use psutil::process;


pub fn ps(is_all: bool) {
    let process = process::processes();
    match &process.is_err() {
        true => {
            panic!("Unable to get process list")
        }
        _ => {}
    }
    for p in &process.unwrap() {
        match p {
            Err(e) => match is_all {
                true => {
                    eprintln!("Unable to get one of process information: {:?}", e)
                }
                _ => {}
            },
            _ => {
                let a_process = p.as_ref().unwrap();
                println!(
                    "pid: {} name: {}",
                    a_process.pid(),
                    a_process.name().unwrap()
                )
            }
        }
    }
}
