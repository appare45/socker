use nix::unistd::Pid;

pub struct Process {
    pid: u32,
    namespace: crate::namespace::Namespace,
}

impl Process {
    pub fn new(pid: u32, namespace: crate::namespace::Namespace) -> Self {
        Self { pid, namespace }
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn namespace(&self) -> &crate::namespace::Namespace {
        &self.namespace
    }
}

const CHILD_STACK_SIZE: usize = 1024 * 1024;

// FlagはDTOで共有したほうが良いのだと思うが、一旦こうしておく
pub fn clone(child_fn: fn(), flags: nix::sched::CloneFlags) -> Result<Pid, nix::Error> {
    let stack: &mut [u8; CHILD_STACK_SIZE] = &mut [0; CHILD_STACK_SIZE];
    let cb = Box::new(move || {
        child_fn();
        0
    });
    let raw = Box::into_raw(cb);
    Ok(unsafe { nix::sched::clone(nix::sched::CloneCb::from_raw(raw), stack, flags, None) }?)
}
