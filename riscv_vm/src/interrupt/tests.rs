use std::{rc::Rc, sync::Mutex, thread, time::Duration};

use enumflags2::BitFlag;

use crate::{
    hart::{privilege::PrivilegeMode, CsrProvider, Hart, MiReg},
    memory::memory_buffer::MemoryBuffer,
    trap::InterruptInternal,
};

use super::{
    imsic::Imsic,
    swi_controller::SwiController,
    timer::{MTimer, TimerRef},
};

#[test]
fn timer() {
    let mut timer = MTimer::new(1);
    let bits = Rc::new(Mutex::new(InterruptInternal::empty()));
    timer.add_interrupt_bits(0, bits.clone());

    const DELAY: u64 = 5_000;

    let time = timer.get_ref().get_time();
    timer.set_cmp_micros(time + DELAY, 0);

    thread::sleep(Duration::from_micros(DELAY));

    timer.generate_interrupts();
    assert!(bits
        .lock()
        .unwrap()
        .contains(InterruptInternal::MachineTimer));
}

#[test]
fn timer_mem_buf() {
    let mut timer = MTimer::new(1);
    let bits = Rc::new(Mutex::new(InterruptInternal::empty()));
    timer.add_interrupt_bits(0, bits.clone());

    const DELAY: u64 = 5_000;

    let mut buf = [0u8; 8];
    buf.copy_from_slice(&timer.read_bytes(0x0u64.into(), 8).unwrap());
    let time = u64::from_le_bytes(buf);
    timer
        .write_bytes(&(time + DELAY).to_le_bytes(), 0x8u64.into())
        .unwrap();

    thread::sleep(Duration::from_micros(DELAY));

    timer.generate_interrupts();
    assert!(bits
        .lock()
        .unwrap()
        .contains(InterruptInternal::MachineTimer));
}

#[test]
fn mswic() {
    let hart = Hart::new(0, Default::default(), TimerRef::dummy());
    let harts = &[hart];
    let mut mswic = SwiController::new(harts, PrivilegeMode::Machine);

    mswic
        .write_bytes(&0x3u64.to_le_bytes(), 0x0u64.into())
        .unwrap();

    assert_eq!(mswic.read_bytes(0x0u64.into(), 1).unwrap(), vec![0x1u8]);

    assert!(harts[0]
        .get_mip_ref()
        .lock()
        .unwrap()
        .contains(InterruptInternal::MachineSoftware));
}

#[test]
fn sswic() {
    let hart = Hart::new(0, Default::default(), TimerRef::dummy());
    let harts = &[hart];
    let mut sswic = SwiController::new(harts, PrivilegeMode::Supervisor);

    sswic
        .write_bytes(&0x3u64.to_le_bytes(), 0x0u64.into())
        .unwrap();

    assert_eq!(sswic.read_bytes(0x0u64.into(), 1).unwrap(), vec![0x1u8]);

    assert!(harts[0]
        .get_mip_ref()
        .lock()
        .unwrap()
        .contains(InterruptInternal::SupervisorSoftware));
}

#[test]
fn imsic() {
    let mip = Rc::new(Mutex::new(InterruptInternal::empty()));
    let mut imsic = Imsic::new(mip.clone());

    imsic.m_seteipnum(15);
    assert_eq!(imsic.get_csr(0x35Cu16.into()), Some(0));

    imsic.write_mireg(MiReg::MiReg1, 0x70, 0b1, false).unwrap();
    imsic
        .write_mireg(MiReg::MiReg1, 0xC0, 0b1 << 15, false)
        .unwrap();

    assert_eq!(imsic.get_csr(0x35Cu16.into()), Some((15 << 16) | 15));
    assert!(mip
        .lock()
        .unwrap()
        .contains(InterruptInternal::MachineExternal));

    imsic.write_mireg(MiReg::MiReg1, 0x72, 10, false).unwrap();

    assert_eq!(imsic.get_csr(0x35Cu16.into()), Some(0));
    assert!(!mip
        .lock()
        .unwrap()
        .contains(InterruptInternal::MachineExternal));

    imsic.write_mireg(MiReg::MiReg1, 0x72, 0, false).unwrap();

    assert_eq!(
        imsic.write_csr(0x35Cu16.into(), 0x0, true).unwrap(),
        Some((15 << 16) | 15)
    );
    assert_eq!(imsic.get_csr(0x35Cu16.into()), Some(0));
    assert!(!mip
        .lock()
        .unwrap()
        .contains(InterruptInternal::MachineExternal));

    // Supervisor

    imsic.s_seteipnum(15);
    assert_eq!(imsic.get_csr(0x15Cu16.into()), Some(0));

    imsic.write_sireg(MiReg::MiReg1, 0x70, 0b1, false).unwrap();
    imsic
        .write_sireg(MiReg::MiReg1, 0xC0, 0b1 << 15, false)
        .unwrap();

    assert_eq!(imsic.get_csr(0x15Cu16.into()), Some((15 << 16) | 15));
    assert!(mip
        .lock()
        .unwrap()
        .contains(InterruptInternal::SupervisorExternal));

    imsic.write_sireg(MiReg::MiReg1, 0x72, 10, false).unwrap();

    assert_eq!(imsic.get_csr(0x15Cu16.into()), Some(0));
    assert!(!mip
        .lock()
        .unwrap()
        .contains(InterruptInternal::SupervisorExternal));

    imsic.write_sireg(MiReg::MiReg1, 0x72, 0, false).unwrap();

    assert_eq!(
        imsic.write_csr(0x15Cu16.into(), 0x0, true).unwrap(),
        Some((15 << 16) | 15)
    );
    assert_eq!(imsic.get_csr(0x15Cu16.into()), Some(0));
    assert!(!mip
        .lock()
        .unwrap()
        .contains(InterruptInternal::SupervisorExternal));
}

// #[test]
// fn imsic_page() {
// let imsic = Rc::new(RefCell::new(Imsic::default()));
// }
