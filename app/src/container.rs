use crate::config_parser::Config;
use anyhow::{bail, Result};
use nix::{
    sched::{unshare, CloneFlags},
    sys::wait::waitpid,
    unistd::{execve, fork, write, ForkResult, Pid},
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
                println!("Container has been started with pid: {}", child);
                self.pid = Some(child);
                waitpid(child, None).unwrap();
                Ok(())
            }
            Ok(ForkResult::Child) => {
                // Unsafe to use `println!` (or `unwrap`) here. See Safety.
                write(0, "I'm a container \n".as_bytes()).ok();
                match self.config.process {
                    Some(ref process) => {
                        let cwd = process.get_cwd();
                        let args = process.get_args();
                        let env = process.get_env();
                        execve(cwd, args[..].as_ref(), env[..].as_ref())?;
                        // unshare(CloneFlags::CLONE_NEWNS)?;
                        unsafe {
                            libc::chdir(cwd.as_ptr());
                        }
                    }
                    None => {}
                }
                unshare(CloneFlags::CLONE_NEWNS)?;
                unsafe { libc::_exit(0) };
            }
            Err(_) => bail!("Failed to fork"),
        }
    }
}
