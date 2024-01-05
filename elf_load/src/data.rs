use enumflags2::bitflags;

use crate::error::{ElfHeaderParseError, ProgramHeaderParseError, SectionHeaderParseError};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Bitness {
    B32 = 1,
    B64 = 2,
    B128 = u8::MAX,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Endianess {
    Little = 1,
    Big = 2,
}

impl TryFrom<u8> for Endianess {
    type Error = ElfHeaderParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Endianess::Little),
            2 => Ok(Endianess::Big),
            _ => Err(ElfHeaderParseError::InvalidEndianess),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum AbiType {
    SystemV = 0x00,
    HpUx = 0x01,
    NetBSD = 0x02,
    Linux = 0x03,
    GnuHurd = 0x04,
    Solaris = 0x06,
    AIX = 0x07,
    IRIX = 0x08,
    FreeBSD = 0x09,
    Tru64 = 0x0A,
    NovellModesto = 0x0B,
    OpenBSD = 0x0C,
    OpenVMS = 0x0D,
    NonStopKernel = 0x0E,
    AROS = 0x0F,
    FenixOS = 0x10,
    NuxiCloudABI = 0x11,
    OpenVOS = 0x12,
}

impl TryFrom<u8> for AbiType {
    type Error = ElfHeaderParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(AbiType::SystemV),
            0x01 => Ok(AbiType::HpUx),
            0x02 => Ok(AbiType::NetBSD),
            0x03 => Ok(AbiType::Linux),
            0x04 => Ok(AbiType::GnuHurd),
            0x06 => Ok(AbiType::Solaris),
            0x07 => Ok(AbiType::AIX),
            0x08 => Ok(AbiType::IRIX),
            0x09 => Ok(AbiType::FreeBSD),
            0x0A => Ok(AbiType::Tru64),
            0x0B => Ok(AbiType::NovellModesto),
            0x0C => Ok(AbiType::OpenBSD),
            0x0D => Ok(AbiType::OpenVMS),
            0x0E => Ok(AbiType::NonStopKernel),
            0x0F => Ok(AbiType::AROS),
            0x10 => Ok(AbiType::FenixOS),
            0x11 => Ok(AbiType::NuxiCloudABI),
            0x12 => Ok(AbiType::OpenVOS),
            _ => Err(ElfHeaderParseError::InvalidAbi(value)),
        }
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    None = 0x00,
    Rel = 0x01,
    Exec = 0x02,
    Dyn = 0x03,
    Core = 0x04,
    /// Range 0xFE00..0xFEFF
    Os(u16) = 0xFE00,
    /// Range 0xFF00..0xFFFF
    Proc(u16) = 0xFF00,
}

impl TryFrom<u16> for ObjectType {
    type Error = ElfHeaderParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ObjectType::None),
            0x01 => Ok(ObjectType::Rel),
            0x02 => Ok(ObjectType::Exec),
            0x03 => Ok(ObjectType::Dyn),
            0x04 => Ok(ObjectType::Core),
            0xFE00..=0xFEFF => Ok(ObjectType::Os(value)),
            0xFF00..=0xFFFF => Ok(ObjectType::Proc(value)),
            _ => Err(ElfHeaderParseError::InvalidObjType(value)),
        }
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum ASI {
    None = 0x00,
    ATnT = 0x01,
    SPARC = 0x02,
    X86 = 0x03,
    M68k = 0x04,
    M88k = 0x05,
    IntelMCU = 0x06,
    Intel80860 = 0x07,
    MIPS = 0x08,
    IBMSystem370 = 0x09,
    MIPSRS3000LE = 0x0A,
    HPPARISC = 0x0F,
    Intel80960 = 0x13,
    PowerPC32 = 0x14,
    PowerPC64 = 0x15,
    S390 = 0x16,
    IBMSPUSPC = 0x17,
    NECV800 = 0x24,
    FujitsuFR20 = 0x25,
    MotorolaRCE = 0x27,
    Arm = 0x28,
    DigitalAlpha = 0x29,
    SuperH = 0x2A,
    SPARCV9 = 0x2B,
    SiemensTRiCore = 0x2C,
    ArgonautRISC = 0x2D,
    HitachiH8300 = 0x2E,
    HitachiH8300H = 0x2F,
    HitachiH8S = 0x30,
    HitachiH8500 = 0x31,
    IA64 = 0x32,
    StanfordMIPSX = 0x33,
    MotorolaColfFire = 0x34,
    MotorolaM68HC12 = 0x35,
    FujitsuMMA = 0x36,
    SiemensPCP = 0x37,
    SonynCPUembeddedRISC = 0x38,
    DensoNDR1 = 0x39,
    MotorolaStar = 0x3A,
    ToyotaME16 = 0x3B,
    STMicroelectronicsST100 = 0x3C,
    AdvancedLogicCorpTinyJ = 0x3D,
    AMDx86_64 = 0x3E,
    SonyDSP = 0x3F,
    DigitalEquipmentCorpPDP10 = 0x40,
    DigitalEquipmentCorpPDP11 = 0x41,
    SiemensFX66 = 0x42,
    STMicroelectronicsST9_8_16bit = 0x43,
    STMicroelectronicsST7_8bit = 0x44,
    MotorolaMC68HC16 = 0x45,
    MotorolaMC68HC11 = 0x46,
    MotorolaMC68HC08 = 0x47,
    MotorolaMC68HC05 = 0x48,
    SiliconGraphicsSVx = 0x49,
    STMicroelectronicsST19_8bit = 0x4A,
    DigitalVAX = 0x4B,
    AxisCommunications32bit = 0x4C,
    InfineonTechnologies32bit = 0x4D,
    Element14_64bitDSP = 0x4E,
    LSILogic16bitDSP = 0x4F,
    TMS320C6000 = 0x8C,
    MCSTElbruse2k = 0xAF,
    Arm64bits = 0xB7,
    ZilogZ80 = 0xDC,
    RISCV = 0xF3,
    BerkeleyPacketFilter = 0xF7,
    WDC65C816 = 0x101,
}

impl TryFrom<u16> for ASI {
    type Error = ElfHeaderParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ASI::None),
            0x01 => Ok(ASI::ATnT),
            0x02 => Ok(ASI::SPARC),
            0x03 => Ok(ASI::X86),
            0x04 => Ok(ASI::M68k),
            0x05 => Ok(ASI::M88k),
            0x06 => Ok(ASI::IntelMCU),
            0x07 => Ok(ASI::Intel80860),
            0x08 => Ok(ASI::MIPS),
            0x09 => Ok(ASI::IBMSystem370),
            0x0A => Ok(ASI::MIPSRS3000LE),
            0x0B..=0x0E => Err(ElfHeaderParseError::ReservedASI),
            0x0F => Ok(ASI::HPPARISC),
            0x13 => Ok(ASI::Intel80960),
            0x14 => Ok(ASI::PowerPC32),
            0x15 => Ok(ASI::PowerPC64),
            0x16 => Ok(ASI::S390),
            0x17 => Ok(ASI::IBMSPUSPC),
            0x18..=0x23 => Err(ElfHeaderParseError::ReservedASI),
            0x24 => Ok(ASI::NECV800),
            0x25 => Ok(ASI::FujitsuFR20),
            0x27 => Ok(ASI::MotorolaRCE),
            0x28 => Ok(ASI::Arm),
            0x29 => Ok(ASI::DigitalAlpha),
            0x2A => Ok(ASI::SuperH),
            0x2B => Ok(ASI::SPARCV9),
            0x2C => Ok(ASI::SiemensTRiCore),
            0x2D => Ok(ASI::ArgonautRISC),
            0x2E => Ok(ASI::HitachiH8300),
            0x2F => Ok(ASI::HitachiH8300H),
            0x30 => Ok(ASI::HitachiH8S),
            0x31 => Ok(ASI::HitachiH8500),
            0x32 => Ok(ASI::IA64),
            0x33 => Ok(ASI::StanfordMIPSX),
            0x34 => Ok(ASI::MotorolaColfFire),
            0x35 => Ok(ASI::MotorolaM68HC12),
            0x36 => Ok(ASI::FujitsuMMA),
            0x37 => Ok(ASI::SiemensPCP),
            0x38 => Ok(ASI::SonynCPUembeddedRISC),
            0x39 => Ok(ASI::DensoNDR1),
            0x3A => Ok(ASI::MotorolaStar),
            0x3B => Ok(ASI::ToyotaME16),
            0x3C => Ok(ASI::STMicroelectronicsST100),
            0x3D => Ok(ASI::AdvancedLogicCorpTinyJ),
            0x3E => Ok(ASI::AMDx86_64),
            0x3F => Ok(ASI::SonyDSP),
            0x40 => Ok(ASI::DigitalEquipmentCorpPDP10),
            0x41 => Ok(ASI::DigitalEquipmentCorpPDP11),
            0x42 => Ok(ASI::SiemensFX66),
            0x43 => Ok(ASI::STMicroelectronicsST9_8_16bit),
            0x44 => Ok(ASI::STMicroelectronicsST7_8bit),
            0x45 => Ok(ASI::MotorolaMC68HC16),
            0x46 => Ok(ASI::MotorolaMC68HC11),
            0x47 => Ok(ASI::MotorolaMC68HC08),
            0x48 => Ok(ASI::MotorolaMC68HC05),
            0x49 => Ok(ASI::SiliconGraphicsSVx),
            0x4A => Ok(ASI::STMicroelectronicsST19_8bit),
            0x4B => Ok(ASI::DigitalVAX),
            0x4C => Ok(ASI::AxisCommunications32bit),
            0x4D => Ok(ASI::InfineonTechnologies32bit),
            0x4E => Ok(ASI::Element14_64bitDSP),
            0x4F => Ok(ASI::LSILogic16bitDSP),
            0x8C => Ok(ASI::TMS320C6000),
            0xAF => Ok(ASI::MCSTElbruse2k),
            0xB7 => Ok(ASI::Arm64bits),
            0xDC => Ok(ASI::ZilogZ80),
            0xF3 => Ok(ASI::RISCV),
            0xF7 => Ok(ASI::BerkeleyPacketFilter),
            0x101 => Ok(ASI::WDC65C816),
            _ => Err(ElfHeaderParseError::InvalidASI),
        }
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum ProgramType {
    Null = 0x00000000,
    Load = 0x00000001,
    Dynamic = 0x00000002,
    Interp = 0x00000003,
    Note = 0x00000004,
    /// range 0x60000000..0x6FFFFFFF
    Os(u32) = 0x60000000,
    /// range 0x70000000..0x7FFFFFFF
    Proc(u32) = 0x70000000,
}

impl TryFrom<u32> for ProgramType {
    type Error = ProgramHeaderParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x00000000 => Ok(ProgramType::Null),
            0x00000001 => Ok(ProgramType::Load),
            0x00000002 => Ok(ProgramType::Dynamic),
            0x00000003 => Ok(ProgramType::Interp),
            0x00000004 => Ok(ProgramType::Note),
            0x60000000..=0x6FFFFFFF => Ok(ProgramType::Os(value)),
            0x70000000..=0x7FFFFFFF => Ok(ProgramType::Proc(value)),
            i => Err(ProgramHeaderParseError::InvalidProgramType(i)),
        }
    }
}

#[repr(u32)]
#[bitflags]
#[derive(Clone, Copy, Debug)]
pub enum ProgramFlags {
    Exec = 0x1,
    Write = 0x2,
    Read = 0x4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    Null = 0x0,
    Progbits = 0x1,
    Symtab = 0x2,
    Strtab = 0x3,
    Rela = 0x4,
    Hash = 0x5,
    Dynamic = 0x6,
    Note = 0x7,
    Nobits = 0x8,
    Rel = 0x9,
    Shlib = 0x0A,
    Dynsym = 0x0B,
    InitArray = 0x0E,
    FiniArray = 0x0F,
    PreinitArray = 0x10,
    Group = 0x11,
    SymtabShndx = 0x12,
    Num = 0x13,
    /// range 0x60000000..u32::MAX
    Os(u32) = 0x60000000,
}

impl TryFrom<u32> for SectionType {
    type Error = SectionHeaderParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SectionType::Null),
            0x1 => Ok(SectionType::Progbits),
            0x2 => Ok(SectionType::Symtab),
            0x3 => Ok(SectionType::Strtab),
            0x4 => Ok(SectionType::Rela),
            0x5 => Ok(SectionType::Hash),
            0x6 => Ok(SectionType::Dynamic),
            0x7 => Ok(SectionType::Note),
            0x8 => Ok(SectionType::Nobits),
            0x9 => Ok(SectionType::Rel),
            0x0A => Ok(SectionType::Shlib),
            0x0B => Ok(SectionType::Dynsym),
            0x0E => Ok(SectionType::InitArray),
            0x0F => Ok(SectionType::FiniArray),
            0x10 => Ok(SectionType::PreinitArray),
            0x11 => Ok(SectionType::Group),
            0x12 => Ok(SectionType::SymtabShndx),
            0x13 => Ok(SectionType::Num),
            0x60000000..=u32::MAX => Ok(SectionType::Os(value)),
            _ => Err(SectionHeaderParseError::InvalidSectionType(value)),
        }
    }
}

#[repr(u64)]
#[bitflags]
#[derive(Clone, Copy, Debug)]
pub enum SectionFlags {
    Write = 0x1,
    Alloc = 0x2,
    Execinstr = 0x4,
    Merge = 0x10,
    Strings = 0x20,
    InfoLink = 0x40,
    LinkOrder = 0x80,
    OsNonconformin = 0x100,
    Group = 0x200,
    Tls = 0x400,
}
