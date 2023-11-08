use std::path::Path;

use nix::mount::{mount, MsFlags};

pub fn mount_bind(src: &str, to: &str) {
    let src_dir = Path::new(src);
    if !src_dir.is_dir() {
        eprintln!("{} is not directory", src);
        return;
    }

    let to_dir = Path::new(to);
    if !to_dir.is_dir() {
        eprintln!("{} is not directory", to);
    }

    mount(
        Some(src_dir),
        to_dir,
        Some(Path::new("")),
        MsFlags::MS_BIND,
        Some(Path::new("")),
    )
    .unwrap();
}
