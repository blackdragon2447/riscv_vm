use enumflags2::bitflags;

#[bitflags]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Counters {
    Cycle = 1 << 0,
    Timer = 1 << 1,
    InstRet = 1 << 2,
    HPM3 = 1 << 3,
    HPM4 = 1 << 4,
    HPM5 = 1 << 5,
    HPM6 = 1 << 6,
    HPM7 = 1 << 7,
    HPM8 = 1 << 8,
    HPM9 = 1 << 9,
    HPM10 = 1 << 10,
    HPM11 = 1 << 11,
    HPM12 = 1 << 12,
    HPM13 = 1 << 13,
    HPM14 = 1 << 14,
    HPM15 = 1 << 15,
    HPM16 = 1 << 16,
    HPM17 = 1 << 17,
    HPM18 = 1 << 18,
    HPM19 = 1 << 19,
    HPM20 = 1 << 20,
    HPM21 = 1 << 21,
    HPM22 = 1 << 22,
    HPM23 = 1 << 23,
    HPM24 = 1 << 24,
    HPM25 = 1 << 25,
    HPM26 = 1 << 26,
    HPM27 = 1 << 27,
    HPM28 = 1 << 28,
    HPM29 = 1 << 29,
    HPM30 = 1 << 30,
    HPM31 = 1 << 31,
}
