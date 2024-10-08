use crate::config_parser::Config;
use anyhow::{bail, Result};
use log::info;
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
    status: ContainerStatus,
    config: Config,
    pid: Option<Pid>,
}

pub trait ContainerTask {
    fn run(&self) -> Result<()>;
}

impl Container {
    pub fn new(config: Config) -> Self {
        Container {
            status: ContainerStatus::Created,
            pid: None,
            config,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.status = ContainerStatus::Running;
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                info!("Container has been started with pid: {}", child);
                self.pid = Some(child);
                waitpid(child, None).unwrap();
                Ok(())
            }
            Ok(ForkResult::Child) => {
                write(0, "I'm a container \n".as_bytes()).ok();
                if let Some(ref root) = self.config.root {
                    root.run()?;
                }
                if let Some(hostname) = &self.config.hostname {
                    hostname.run()?;
                }
                if let Some(mounts) = &self.config.mounts {
                    for mount in mounts {
                        mount.run()?;
                    }
                }
                if let Some(process) = &self.config.process {
                    process.run()?;
                }
                Ok(())
            }
            Err(_) => bail!("Failed to fork"),
        }
    }
}
