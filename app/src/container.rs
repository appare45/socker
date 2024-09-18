use crate::config_parser::Config;
use anyhow::{bail, Result};
use nix::{
    sys::wait::waitpid,
    unistd::{fork, write, ForkResult, Pid},
};

// not implemented all variantes
enum ContainerStatus {
    Created,
    Running,
}

// not implemented all variantes
pub struct Container {
    pub version: String,
    status: ContainerStatus,
    pid: Option<Pid>,
}

impl Container {
    pub fn new(config: Config) -> Self {
        Container {
            version: config.oci_version,
            status: ContainerStatus::Created,
            pid: None,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.status = ContainerStatus::Running;
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                println!("Container has been started with pid: {}", child);
                self.pid = Some(child);
                waitpid(child, None).unwrap();
                Ok(())
            }
            Ok(ForkResult::Child) => {
                // Unsafe to use `println!` (or `unwrap`) here. See Safety.
                write(0, "I'm a container \n".as_bytes()).ok();
                unsafe { libc::_exit(0) };
            }
            Err(_) => bail!("Failed to fork"),
        }
    }
}
