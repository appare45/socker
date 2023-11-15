use nix::{
    libc::{self, SIGCHLD},
    sched::{clone, CloneFlags},
    sys::wait::waitpid,
    unistd::{chroot, execvp},
};

use crate::mount::mount::mount_bind;

use std::{
    ffi::CString,
    path::Path,
};


pub fn new_uts() {
    let root = Path::new("./test-new");
    let src = Path::new("./debian");
    let closure = || {
        // 現在のディレクトリをバインドマウント
        mount_bind(src, root);
        println!("new root from: {}", root.display());
        chroot(root).unwrap();
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
    let flags = CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_FILES;
    let signal = SIGCHLD as libc::c_int;

    let pid = clone(cb, &mut stack, flags, Some(signal));
    println!("PID: {:?}", pid);

    while let Ok(status) = waitpid(None, None) {
        println!("Exit Status: {:?}", status)
    }

}
