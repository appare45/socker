use nix::unistd::execv;
use std::ffi::CString;
use nix::sched::sched_yield;
pub fn clone() {
    let cb = Box::new(|| {
        let cmd = CString::new("bash").unwrap();
        let args = vec![CString::new("-l").unwrap()];
        if let Err(e) = execv(&cmd, &args.as_ref()) {
            eprintln!("Unable to excute bash");
            return 127;
        }
        return 127;
    });
    
    let mut child_stack = [0u8; 8192];
    
}
