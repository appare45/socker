pub mod mount {
    use nix::mount::{mount, MsFlags};
    use std::path::Path;
    pub fn mount_bind(src: &Path, to: &Path) {
        let src_dir = Path::new(src);
        if !src_dir.is_dir() {
            eprintln!("{} is not directory", src.display());
            return;
        }

        let to_dir = Path::new(to);
        if !to_dir.is_dir() {
            eprintln!("{} is not directory", to.display());
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
}
