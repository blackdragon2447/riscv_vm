use enumflags2::{bitflags, make_bitflags, BitFlag, BitFlags};

use crate::{
    hart::privilege::{self, PrivilegeMode},
    memory::pmp::AccessMode,
};

use super::{
    address::{Address, VirtAddress},
    MemoryError, MemoryWindow,
};

const PAGE_SIZE: u64 = 4096;
const PTE_SIZE: u64 = 8;

#[derive(Debug, Clone, Copy)]
pub struct Satp {
    pub mode: AddressTranslationMode,
    // asid: 0,
    pub ppn: u64, // bits 0..=43
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressTranslationMode {
    Bare = 0,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
}

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PteFlags {
    V = 0b1 << 0,
    R = 0b1 << 1,
    W = 0b1 << 2,
    X = 0b1 << 3,
    U = 0b1 << 4,
    G = 0b1 << 5,
    A = 0b1 << 6,
    D = 0b1 << 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pte {
    Sv39 {
        flags: BitFlags<PteFlags>,
        ppn: [u32; 3], // 9 9 26
        pbmt: u8,      // Svbpmt ext
        n: bool,       // SvNapot ext
    },
    Sv48 {
        flags: BitFlags<PteFlags>,
        ppn: [u32; 4], // 9 9 9 17
        pbmt: u8,      // Svbpmt ext
        n: bool,       // SvNapot ext
    },
    Sv57 {
        flags: BitFlags<PteFlags>,
        ppn: [u32; 5], // 9 9 9 9 8
        pbmt: u8,      // Svbpmt ext
        n: bool,       // SvNapot ext
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PteType {
    Branch(Pte),
    Leaf(Pte),
}

pub enum PageError {
    AccessFault,
    PageFault,
}

pub struct AccessContext {
    pub mode: AccessMode,
    pub privilege: PrivilegeMode,
    pub mxr: bool,
    pub sum: bool,
}

pub fn walk_page_table(
    virt: VirtAddress,
    satp: Satp,
    mem: &MemoryWindow,
    context: AccessContext,
) -> Result<Address, PageError> {
    if context.privilege == PrivilegeMode::Machine {
        panic!("Why are you paging in M mode, you dummy");
    }
    let base = satp.ppn * PAGE_SIZE;

    match satp.mode {
        m @ AddressTranslationMode::Sv39 => {
            walk_page_table_internal(virt, base, 3, 2, mem, m, context)
        }
        m @ AddressTranslationMode::Sv48 => {
            walk_page_table_internal(virt, base, 4, 3, mem, m, context)
        }
        m @ AddressTranslationMode::Sv57 => {
            walk_page_table_internal(virt, base, 5, 4, mem, m, context)
        }
        AddressTranslationMode::Bare => {
            unreachable!("We should not attempt to walk the page table it it is bare")
        }
    }
}

fn walk_page_table_internal(
    virt: VirtAddress,
    base: u64,
    levels: usize,
    i: usize,
    mem: &MemoryWindow,
    mode: AddressTranslationMode,
    context: AccessContext,
) -> Result<Address, PageError> {
    let pte = PteType::from_bytes(
        u64::from_le_bytes(
            mem.read_phys(
                (base + (virt.vpn[i] as u64) * PTE_SIZE).into(),
                PTE_SIZE as usize,
            )?
            .try_into()
            .unwrap(),
        ),
        mode,
    );

    match pte {
        PteType::Branch(pte) => {
            if !pte.flags().contains(PteFlags::V) {
                return Err(PageError::PageFault);
            }
            walk_page_table_internal(
                virt,
                pte.ppn() * PAGE_SIZE,
                levels,
                i - 1,
                mem,
                mode,
                context,
            )
        }
        PteType::Leaf(pte) => {
            if (pte.flags().contains(PteFlags::U)
                && (context.privilege == PrivilegeMode::User || context.sum))
                || (!pte.flags().contains(PteFlags::U)
                    && context.privilege == PrivilegeMode::Supervisor)
            {
                match context.mode {
                    AccessMode::Read => {
                        if !(pte.flags().contains(PteFlags::R)
                            || (pte.flags().contains(PteFlags::X) && context.mxr))
                        {
                            return Err(PageError::PageFault);
                        }
                    }
                    AccessMode::Write => {
                        #[allow(clippy::if_same_then_else)] // Make intent mode clear
                        if !pte.flags().contains(PteFlags::W) {
                            return Err(PageError::PageFault);
                        } else if !pte.flags().contains(PteFlags::D) {
                            return Err(PageError::PageFault);
                        }
                    }
                    AccessMode::Exec => {
                        if !pte.flags().contains(PteFlags::X)
                            || (pte.flags().contains(PteFlags::U)
                                && context.privilege == PrivilegeMode::Supervisor)
                        {
                            return Err(PageError::PageFault);
                        }
                    }
                }
                if !pte.flags().contains(PteFlags::A) {
                    return Err(PageError::PageFault);
                }
                if i != 0 {
                    let ppn = pte.ppn_bytes();
                    for j in ppn.iter().take(i) {
                        if *j != 0 {
                            return Err(PageError::PageFault);
                        }
                    }
                }
                let mut ppn = 0u64;
                for j in 0..i {
                    ppn |= (virt.vpn[j] as u64) << (j * 9);
                }
                for j in i..levels {
                    ppn |= (pte.ppn_bytes()[j] as u64) << (j * 9);
                }
                Ok(((virt.page_offset as u64) | (ppn << 12)).into())
            } else {
                Err(PageError::PageFault)
            }
        }
    }
}

impl Pte {
    pub fn flags(&self) -> &BitFlags<PteFlags> {
        match self {
            Pte::Sv39 { flags, .. } => flags,
            Pte::Sv48 { flags, .. } => flags,
            Pte::Sv57 { flags, .. } => flags,
        }
    }

    pub fn ppn(&self) -> u64 {
        match self {
            Pte::Sv39 { ppn, .. } => (ppn[0] as u64) | (ppn[1] as u64) << 9 | (ppn[2] as u64) << 18,
            Pte::Sv48 { ppn, .. } => {
                (ppn[0] as u64)
                    | (ppn[1] as u64) << 9
                    | (ppn[2] as u64) << 18
                    | (ppn[3] as u64) << 27
            }
            Pte::Sv57 { ppn, .. } => {
                (ppn[0] as u64)
                    | (ppn[1] as u64) << 9
                    | (ppn[2] as u64) << 18
                    | (ppn[3] as u64) << 27
                    | (ppn[4] as u64) << 36
            }
        }
    }

    pub fn ppn_bytes(&self) -> &[u32] {
        match self {
            Pte::Sv39 { ppn, .. } => ppn,
            Pte::Sv48 { ppn, .. } => ppn,
            Pte::Sv57 { ppn, .. } => ppn,
        }
    }
}

impl PteType {
    pub fn from_bytes(bits: u64, mode: AddressTranslationMode) -> Self {
        match mode {
            AddressTranslationMode::Sv39 => {
                // we may unwrap since all possible values of u8 are covered
                let flags = BitFlags::<PteFlags>::from_bits(bits as u8).unwrap();
                let mask_9 = 0x1FF;
                let mask_26 = 0x3FFFFFF;
                let mut ppn = [0u32; 3];
                ppn[0] = ((bits >> 10) & mask_9) as u32;
                ppn[1] = ((bits >> 19) & mask_9) as u32;
                ppn[2] = ((bits >> 28) & mask_26) as u32;
                // pbmt rw 0
                // n rw 0
                if (flags & make_bitflags!(PteFlags::{R | W | X})) == PteFlags::empty() {
                    PteType::Branch(Pte::Sv39 {
                        flags,
                        ppn,
                        pbmt: 0,
                        n: false,
                    })
                } else {
                    PteType::Leaf(Pte::Sv39 {
                        flags,
                        ppn,
                        pbmt: 0,
                        n: false,
                    })
                }
            }
            AddressTranslationMode::Sv48 => {
                // we may unwrap since all possible values of u8 are covered
                let flags = BitFlags::<PteFlags>::from_bits(bits as u8).unwrap();
                let mask_9 = 0x1FF;
                let mask_17 = 0x1FFFF;
                let mut ppn = [0u32; 4];
                ppn[0] = ((bits >> 10) & mask_9) as u32;
                ppn[1] = ((bits >> 19) & mask_9) as u32;
                ppn[2] = ((bits >> 28) & mask_9) as u32;
                ppn[3] = ((bits >> 37) & mask_17) as u32;
                // pbmt rw 0
                // n rw 0
                if (flags & make_bitflags!(PteFlags::{R | W | X})) == PteFlags::empty() {
                    PteType::Branch(Pte::Sv48 {
                        flags,
                        ppn,
                        pbmt: 0,
                        n: false,
                    })
                } else {
                    PteType::Leaf(Pte::Sv48 {
                        flags,
                        ppn,
                        pbmt: 0,
                        n: false,
                    })
                }
            }
            AddressTranslationMode::Sv57 => {
                // we may unwrap since all possible values of u8 are covered
                let flags = BitFlags::<PteFlags>::from_bits(bits as u8).unwrap();
                let mask_9 = 0x1FF;
                let mask_8 = 0xFF;
                let mut ppn = [0u32; 5];
                ppn[0] = ((bits >> 10) & mask_9) as u32;
                ppn[1] = ((bits >> 19) & mask_9) as u32;
                ppn[2] = ((bits >> 28) & mask_9) as u32;
                ppn[3] = ((bits >> 37) & mask_9) as u32;
                ppn[4] = ((bits >> 46) & mask_8) as u32;
                // pbmt rw 0
                // n rw 0
                if (flags & make_bitflags!(PteFlags::{R | W | X})) == PteFlags::empty() {
                    PteType::Branch(Pte::Sv57 {
                        flags,
                        ppn,
                        pbmt: 0,
                        n: false,
                    })
                } else {
                    PteType::Leaf(Pte::Sv57 {
                        flags,
                        ppn,
                        pbmt: 0,
                        n: false,
                    })
                }
            }
            _ => unreachable!("We should never attempt to read the pt if it is bare"),
        }
    }
}

impl Satp {
    pub fn to_bits(&self) -> u64 {
        let mut bits = self.ppn;

        bits |= ((self.mode as u8 as u64) << 60);

        bits
    }

    pub fn from_bits(bits: u64) -> Option<Self> {
        let mode = (bits >> 60).try_into().ok()?;
        Some(Self {
            mode,
            ppn: bits & 0xFFF_FFFF_FFFF,
        })
    }
}

impl TryFrom<u64> for AddressTranslationMode {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Bare),
            8 => Ok(Self::Sv39),
            9 => Ok(Self::Sv48),
            10 => Ok(Self::Sv57),
            _ => Err(()),
        }
    }
}

impl From<MemoryError> for PageError {
    fn from(value: MemoryError) -> Self {
        match value {
            MemoryError::OutOfBoundsWrite(_) => unreachable!(),
            MemoryError::OutOfBoundsRead(_) => Self::AccessFault,
            MemoryError::OutOfMemory => unreachable!(),
            MemoryError::PmpDeniedRead => Self::AccessFault,
            MemoryError::PmpDeniedWrite => unreachable!(),
            MemoryError::PageFaultRead => Self::PageFault,
            MemoryError::PageFaultWrite => Self::PageFault,
            MemoryError::DeviceMemoryPoison => panic!("DeviceMemoryPoison"),
            MemoryError::PmpDeniedFetch => Self::AccessFault,
            MemoryError::PageFaultFetch => Self::PageFault,
        }
    }
}
