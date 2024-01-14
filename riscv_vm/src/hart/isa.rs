use enumflags2::{bitflags, BitFlags};

#[repr(u64)]
#[bitflags]
#[derive(Clone, Copy, Debug)]
pub enum Isa {
    A = 0x1,
    B = 0x2,
    C = 0x4,
    D = 0x8,
    E = 0x10,
    F = 0x20,
    H = 0x40,
    I = 0x80,
    J = 0x100,
    K = 0x200,
    L = 0x400,
    M = 0x800,
    N = 0x1000,
    O = 0x2000,
    P = 0x4000,
    Q = 0x8000,
    R = 0x10000,
    S = 0x20000,
    T = 0x40000,
    U = 0x80000,
    V = 0x100000,
    W = 0x200000,
    X = 0x400000,
    Y = 0x800000,
    Z = 0x1000000,
}

impl Isa {
    pub fn maximal() -> BitFlags<Self> {
        Self::I | Self::M | Self::S | Self::U
    }

    pub fn validate(bitflags: &mut BitFlags<Self>) {
        *bitflags &= Self::maximal();
    }
}
