use std::{
    fs,
    io::{self, PipeReader, Read, Write},
    os::fd::{AsRawFd, FromRawFd},
};

use nix::{
    libc::SIGCHLD,
    unistd::{Gid, Uid, close, setgid, setuid},
};
use socker::namespace::flags::NamespaceFlags;

#[derive(Clone, Copy)]
struct Context {
    readerfd: i32,
    writerfd: i32,
    uid: Uid,
    gid: Gid,
}

fn child_fn(ctx: Context) {
    let pid = nix::unistd::getpid();
    println!("In child process with PID: {}", pid);
    let mut reader = unsafe { PipeReader::from_raw_fd(ctx.readerfd) };
    close(ctx.writerfd).expect("Failed to close write end of pipe in child");
    let uid = nix::unistd::getuid();
    let gid = nix::unistd::getgid();
    println!("Child before mapping UID: {}, GID: {}", uid, gid);
    let mut read = [0_u8; 128];
    reader.read(&mut read).unwrap();
    println!("Read: {}", String::from_utf8_lossy(&read));
    setuid(ctx.uid).expect("Failed to set UID to 0");
    setgid(ctx.gid).expect("Failed to set GID to 0");
    let uid = nix::unistd::getuid();
    let gid = nix::unistd::getgid();
    println!("Child UID: {}, GID: {}", uid, gid);
}
const CHILD_STACK_SIZE: usize = 1024 * 1024;

fn main() {
    println!("Hello, world!");
    let flags = socker::namespace::Namespace::new(NamespaceFlags::USER | NamespaceFlags::PID);
    let (reader, mut writer) = io::pipe().expect("Failed to create pipe");
    let stack: &mut [u8; CHILD_STACK_SIZE] = &mut [0; CHILD_STACK_SIZE];
    let ctx = Context {
        readerfd: reader.as_raw_fd(),
        writerfd: writer.as_raw_fd(),
        uid: Uid::from_raw(0),
        gid: Gid::from_raw(0),
    };
    let pid = match unsafe {
        nix::sched::clone(
            Box::new(move || {
                child_fn(ctx);
                0
            }),
            stack,
            flags.proc_clone_flags(),
            Some(SIGCHLD),
        )
    } {
        Ok(child_pid) => {
            println!("Spawned child process with PID: {}", child_pid);
            child_pid
        }
        Err(e) => {
            eprintln!("Failed to clone process: {}", e);
            return;
        }
    };
    let uid = nix::unistd::getuid();
    let gid = nix::unistd::getgid();
    println!("Parent UID: {}, GID: {}", uid, gid);
    fs::write(format!("/proc/{}/setgroups", pid), "deny\n").expect("Failed to write setgroups");
    // GID map
    fs::write(
        format!("/proc/{}/gid_map", pid),
        format!("0 {} 1\n", gid.as_raw()),
    )
    .expect("Failed to write gid_map");
    // UID map
    fs::write(
        format!("/proc/{}/uid_map", pid),
        format!("0 {} 1\n", uid.as_raw()),
    )
    .expect("Failed to write uid_map");

    println!("Writing to pipe...");
    writer
        .write_all(b"Hello from parent process!\n")
        .expect("Failed to write to pipe");
    match nix::sys::wait::waitpid(pid, None) {
        Ok(status) => {
            println!("Child process {} exited with status: {:?}", pid, status);
        }
        Err(e) => {
            eprintln!("Failed to wait for child process {}: {}", pid, e);
        }
    }
}
