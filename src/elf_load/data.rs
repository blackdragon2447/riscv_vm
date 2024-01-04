#[repr(u8)]
pub enum Bitness {
    B32 = 1,
    B64 = 2,
    B128 = u8::MAX,
}

#[repr(u8)]
pub enum Endianess {
    Little = 1,
    Big = 2,
}

#[repr(u16)]
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

#[repr(u8)]
pub enum ObjectType {
    None = 0x00,
    Rel = 0x01,
    Exec = 0x02,
    Dyn = 0x03,
    Core = 0x04,
}

#[repr(u16)]
pub enum Architecture {
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

#[repr(u32)]
pub enum ProgramType {
    Null = 0x00000000,
    Load = 0x00000001,
    Dynamic = 0x00000002,
    Interp = 0x00000003,
    Note = 0x00000004,
}

#[repr(u32)]
pub enum ProgramFlags {
    Exec = 0x1,
    Write = 0x2,
    Read = 0x4,
}

#[repr(u32)]
pub enum SectionType {
    Null = 0x0,
    Progbitsp = 0x1,
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
}

#[repr(u64)]
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
