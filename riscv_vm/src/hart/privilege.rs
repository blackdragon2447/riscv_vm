#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrivilegeMode {
    User = 0b00,
    Supervisor = 0b01,
    // Hypervisor = 0b10,
    Machine = 0b11,
}
