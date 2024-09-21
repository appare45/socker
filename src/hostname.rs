use anyhow::{Context, Ok, Result};

use nix::{
    sched::{unshare, CloneFlags},
    unistd::sethostname,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Hostname(String);

impl Hostname {
    pub fn run(&self) -> Result<()> {
        unshare(CloneFlags::CLONE_NEWUTS)
            .context("Failed to unshare UTS for hostname separation")?;
        sethostname(&self.0)?;
        Ok(())
    }
}
