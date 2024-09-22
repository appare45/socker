use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use nix::mount::{mount, MsFlags};
use serde::Deserialize;

use crate::container::ContainerTask;

#[derive(Deserialize)]
pub struct Mounts {
    destination: String,
    source: Option<String>,
    pub options: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub fs_type: Option<String>,
}

impl Mounts {
    pub fn source(&self) -> Option<&Path> {
        self.source.as_ref().map(|e| Path::new(e).as_ref())
    }
    pub fn destination(&self) -> PathBuf {
        PathBuf::from(&self.destination)
    }
    pub fn fs_type(&self) -> Option<&str> {
        self.fs_type.as_deref()
    }
    pub fn options(&self) -> MsFlags {
        let mut flags = MsFlags::empty();
        if let Some(options) = &self.options {
            for option in options {
                match option.as_str() {
                    "bind" => flags |= MsFlags::MS_BIND,
                    "rbind" => flags |= MsFlags::MS_BIND | MsFlags::MS_REC,
                    "ro" => flags |= MsFlags::MS_RDONLY,
                    "rro" => flags |= MsFlags::MS_RDONLY,
                    "nosuid" => flags |= MsFlags::MS_NOSUID,
                    "rnosuid" => flags |= MsFlags::MS_NOSUID | MsFlags::MS_REC,
                    "nodev" => flags |= MsFlags::MS_NODEV,
                    "noexec" => flags |= MsFlags::MS_NOEXEC,
                    "rnoexec" => flags |= MsFlags::MS_NOEXEC | MsFlags::MS_REC,
                    "sync" => flags |= MsFlags::MS_SYNCHRONOUS,
                    "remount" => flags |= MsFlags::MS_REMOUNT,
                    "dirsync" => flags |= MsFlags::MS_DIRSYNC,
                    "noatime" => flags |= MsFlags::MS_NOATIME,
                    "rnoatime" => flags |= MsFlags::MS_NOATIME | MsFlags::MS_REC,
                    "unbundable" => flags |= MsFlags::MS_UNBINDABLE,
                    "runbindable" => flags |= MsFlags::MS_UNBINDABLE | MsFlags::MS_REC,
                    "private" => flags |= MsFlags::MS_PRIVATE,
                    "rprivate" => flags |= MsFlags::MS_PRIVATE | MsFlags::MS_REC,
                    "slave" => flags |= MsFlags::MS_SLAVE,
                    "rslave" => flags |= MsFlags::MS_SLAVE | MsFlags::MS_REC,
                    "shared" => flags |= MsFlags::MS_SHARED,
                    "rshared" => flags |= MsFlags::MS_SHARED | MsFlags::MS_REC,
                    "iversion" => flags |= MsFlags::MS_I_VERSION,
                    "strictatime" => flags |= MsFlags::MS_STRICTATIME,
                    "rstictatime" => flags |= MsFlags::MS_STRICTATIME | MsFlags::MS_REC,
                    "lazytime" => flags |= MsFlags::MS_LAZYTIME,
                    "nodiratime" => flags |= MsFlags::MS_NODIRATIME,
                    _ => (),
                }
            }
        }
        flags
    }
}

impl ContainerTask for Mounts {
    fn run(&self) -> Result<()> {
        let source = self.source();
        let destination = self.destination();
        mount::<Path, PathBuf, str, str>(
            source,
            &destination,
            self.fs_type(),
            self.options(),
            None,
        )
        .context(format!(
            "Failed to mount to {}",
            destination.as_path().display()
        ))?;
        Ok(())
    }
}
