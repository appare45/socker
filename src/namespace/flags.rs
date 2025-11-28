use bitflags::bitflags;
use nix::sched::CloneFlags;

bitflags! {
    #[derive(Default, Clone)]
    pub struct NamespaceFlags: u32 {
        const CGROUP = 0b00000001;
        const IPC    = 0b00000010;
        const MNT    = 0b00000100;
        const NET    = 0b00001000;
        const PID    = 0b00010000;
        const USER   = 0b00100000;
        const UTS    = 0b01000000;
    }
}

impl NamespaceFlags {
    pub fn to_clone_flag(&self) -> CloneFlags {
        let mut c_flag = CloneFlags::empty();
        if self.contains(NamespaceFlags::CGROUP) {
            c_flag |= nix::sched::CloneFlags::CLONE_NEWCGROUP;
        }
        if self.contains(NamespaceFlags::IPC) {
            c_flag |= nix::sched::CloneFlags::CLONE_NEWIPC;
        }
        if self.contains(NamespaceFlags::MNT) {
            c_flag |= nix::sched::CloneFlags::CLONE_NEWNS;
        }
        if self.contains(NamespaceFlags::NET) {
            c_flag |= nix::sched::CloneFlags::CLONE_NEWNET;
        }
        if self.contains(NamespaceFlags::PID) {
            c_flag |= nix::sched::CloneFlags::CLONE_NEWPID;
        }
        if self.contains(NamespaceFlags::USER) {
            c_flag |= nix::sched::CloneFlags::CLONE_NEWUSER;
        }
        if self.contains(NamespaceFlags::UTS) {
            c_flag |= nix::sched::CloneFlags::CLONE_NEWUTS;
        }
        c_flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_clone_flag_combined() {
        let flags = NamespaceFlags::CGROUP | NamespaceFlags::NET | NamespaceFlags::USER;
        let expected = nix::sched::CloneFlags::CLONE_NEWCGROUP
            | nix::sched::CloneFlags::CLONE_NEWNET
            | nix::sched::CloneFlags::CLONE_NEWUSER;
        assert_eq!(flags.to_clone_flag(), expected);
    }

    #[test]
    fn test_to_clone_flag_none() {
        let flags = NamespaceFlags::empty();
        assert_eq!(flags.to_clone_flag(), CloneFlags::empty());
    }
}
