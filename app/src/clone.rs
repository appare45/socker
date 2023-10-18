use nix::{
    sys::wait::waitpid,
    libc::{self, SIGCHLD},
    sched::{clone, CloneFlags},
    unistd::execvp,
};
use std::ffi::CString;
pub fn new_uts() {
    let closure = || {
        let cmd = CString::new("bash").unwrap();
        let args = vec![CString::new("containered bash").unwrap()];
        match execvp(&cmd, &args) {
            Err(e) => {
                eprint!("Error -> {}", e);
                return 256;
            }
            _ => return 256,
        }
    };

    let cb = Box::new(closure);
    let mut stack = [0u8; 8192];
    let flags = CloneFlags::CLONE_NEWUTS;
    let signal = SIGCHLD as libc::c_int;

    let pid = clone(cb, &mut stack, flags, Some(signal));
    println!("PID: {:?}", pid);

    while let Ok(status) = waitpid(None, None) {
        println!("Exit Status: {:?}", status)
    }
}
