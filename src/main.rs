use socker::{namespace::flags::NamespaceFlags, process::clone};

fn child_fn() {
    let pid = nix::unistd::getpid();
    println!("In child process with PID: {}", pid);
}

fn main() {
    println!("Hello, world!");
    let flags = socker::namespace::Namespace::new(NamespaceFlags::PID);
    match clone(child_fn, flags.proc_clone_flags()) {
        Ok(child_pid) => {
            println!("Spawned child process with PID: {}", child_pid);
        }
        Err(e) => {
            eprintln!("Failed to clone process: {}", e);
        }
    }
}
