use anyhow::{ensure, Context, Result};
use log::debug;
use nix::{
    mount::{mount, umount2, MntFlags, MsFlags},
    sched::{unshare, CloneFlags},
    unistd::{chdir, pivot_root},
};
use serde::Deserialize;
use std::{
    env::temp_dir,
    fs::{create_dir, remove_dir, remove_dir_all},
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
pub struct Root {
    pub path: String,
    pub readonly: bool,
}

impl Root {
    fn tmp_path(&self) -> Box<PathBuf> {
        Box::new(temp_dir().join(Path::new(&self.path)))
    }
    fn mount(&self) -> Result<()> {
        debug!("Mounting rootfs");
        let root_path = Path::new(&self.path);
        ensure!(root_path.exists(), "Rootfs path not found");
        let tmp_path = self.tmp_path();
        if tmp_path.exists() {
            debug!("Removing old tmp dir: {:?}", tmp_path);
            remove_dir_all(tmp_path.to_path_buf()).context("Removing old tmp dir failed")?;
        }
        debug!("Creating new tmpfs: {:?}", tmp_path);
        create_dir(tmp_path.to_path_buf()).context("Creating temp dir for mounting root failed")?;
        debug!("Mounting {} to {}", root_path.display(), tmp_path.display());
        mount::<str, str, str, str>(
            Some("none"),
            "/",
            None,
            MsFlags::MS_PRIVATE | MsFlags::MS_REC,
            None,
        )
        .context("Failed to make rootfs private")?;
        mount::<Path, PathBuf, str, str>(
            Some(root_path),
            &tmp_path,
            None,
            if self.readonly {
                MsFlags::MS_BIND | MsFlags::MS_REC | MsFlags::MS_RDONLY
            } else {
                MsFlags::MS_BIND | MsFlags::MS_REC
            },
            None,
        )
        .context("Root dir mount failed")?;
        Ok(())
    }
    pub fn pivot(&self) -> Result<()> {
        debug!("Root service has been started");
        self.mount()?;

        chdir(&self.tmp_path().to_path_buf()).context("Failed to change root dir")?;
        const OLD_ROOT: &str = "old_root";
        let old_root = self.tmp_path().join(OLD_ROOT);
        if !old_root.exists() {
            create_dir(&old_root).context("Failed to create old root dir")?;
        }
        unshare(CloneFlags::CLONE_NEWNS).context("Failed to unshare")?;
        pivot_root::<PathBuf, PathBuf>(&self.tmp_path(), &old_root)
            .context("Failed to pivot root")?;
        mount::<str, str, str, str>(Some("proc"), "/proc", Some("proc"), MsFlags::empty(), None)
            .context("Failed to mount /proc")?;
        umount2(OLD_ROOT, MntFlags::MNT_DETACH).context("Failed to unmount old root")?;
        remove_dir(OLD_ROOT).context("Failed to remove old root dir")?;
        Ok(())
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        if self.tmp_path().exists() {
            umount2(&self.tmp_path().to_path_buf(), MntFlags::MNT_DETACH)
                .expect("Failed to unmount rootfs");
            remove_dir_all(self.tmp_path().to_path_buf()).expect("Failed to remove tmp dir");
        }
        debug!("Root service has been stopped");
    }
}
