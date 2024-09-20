use anyhow::{Context, Ok, Result};
use nix::unistd::execvpe;
use serde::Deserialize;
use std::ffi::CStr;

// Terminal is not supported
#[derive(Deserialize, Clone)]
pub struct Process {
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub cwd: String,
}

impl Process {
    fn get_cmd(&self) -> &CStr {
        match self.args {
            Some(ref args) => unsafe { CStr::from_ptr(args[0].as_ptr() as *const i8) },
            None => unsafe { CStr::from_ptr(self.cwd.as_ptr() as *const i8) },
        }
    }
    fn get_args(&self) -> Vec<&CStr> {
        self.args.clone().unwrap_or(Vec::new())[1..]
            .iter()
            .map(|s| unsafe { CStr::from_ptr(s.as_ptr() as *const i8) })
            .collect::<Vec<&CStr>>()
    }
    fn get_env(&self) -> Vec<&CStr> {
        self.env
            .clone()
            .unwrap_or(Vec::new())
            .iter()
            .map(|s| unsafe { CStr::from_ptr(s.as_ptr() as *const i8) })
            .collect::<Vec<&CStr>>()
    }
    pub fn run(&self) -> Result<()> {
        {
            let cwd = self.get_cmd();
            let args = self.get_args();
            let env = self.get_env();
            execvpe(cwd, args[..].as_ref(), env[..].as_ref()).context("Failed to run process")?;
            Ok(())
        }
    }
}
