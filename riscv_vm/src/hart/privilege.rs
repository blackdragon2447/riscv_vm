#[repr(u8)]
pub enum PrivilegeMode {
    User = 0b00,
    Supervisor = 0b01,
    // Hypervisor = 0b10,
    Machine = 0b11,
}
