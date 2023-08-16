use nix::unistd::{getpid, getppid};
pub fn ps() {
    let current_pid = getpid();
    let current_ppid = getppid();
    println!("Current PID is {}", current_pid.as_raw());
    println!("Current PPID is {}", current_ppid.as_raw())
}
