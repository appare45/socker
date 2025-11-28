use flags::NamespaceFlags;

pub mod flags;

#[derive(Clone, Default)]
pub struct Namespace {
    flags: NamespaceFlags,
}

impl Namespace {
    pub fn new(flags: NamespaceFlags) -> Self {
        Namespace { flags }
    }

    pub fn has_flag(&self, flag: NamespaceFlags) -> bool {
        self.flags.contains(flag)
    }

    pub fn add_flag(&mut self, flag: NamespaceFlags) {
        self.flags.insert(flag);
    }

    pub fn remove_flag(&mut self, flag: NamespaceFlags) {
        self.flags.remove(flag);
    }

    pub fn proc_clone_flags(&self) -> nix::sched::CloneFlags {
        self.flags.to_clone_flag()
    }
}
