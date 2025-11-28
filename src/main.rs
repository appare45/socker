use std::{
    io::{self, PipeReader, Read, Write},
    os::fd::{AsRawFd, FromRawFd},
};

use nix::{libc::SIGCHLD, unistd::close};
use socker::namespace::flags::NamespaceFlags;

fn child_fn(readfd: i32, writefd: i32) {
    let pid = nix::unistd::getpid();
    println!("In child process with PID: {}", pid);
    let mut reader = unsafe { PipeReader::from_raw_fd(readfd) };
    close(writefd).expect("Failed to close write end of pipe in child");
    let mut read = [0_u8; 128];
    reader.read(&mut read).unwrap();
    println!("Read: {}", String::from_utf8_lossy(&read));
}
const CHILD_STACK_SIZE: usize = 1024 * 1024;

fn main() {
    println!("Hello, world!");
    let flags = socker::namespace::Namespace::new(NamespaceFlags::empty());
    let (reader, mut writer) = io::pipe().expect("Failed to create pipe");
    let stack: &mut [u8; CHILD_STACK_SIZE] = &mut [0; CHILD_STACK_SIZE];
    let pid = match unsafe {
        nix::sched::clone(
            Box::new(|| {
                child_fn(reader.as_raw_fd(), writer.as_raw_fd());
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
