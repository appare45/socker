use psutil::process;
pub fn ps() {
    let process = process::processes();
    match &process.is_err() {
       true => {
           panic!("Unable to get process list")
       } 
       _ => {}
    } 
    for p in &process.unwrap() {
       match p.is_err() {
           true => {
            println!("Unable to get one of process information: {:?}", p.as_ref().err())
           }
           _ => {
            println!("name: {}", p.as_ref().unwrap().name().unwrap())
           }
       } 
    }
}
