use std::string::FromUtf8Error;

use enumflags2::{BitFlag, FromBitsError};

#[derive(Debug, PartialEq, Eq)]
pub enum ElfHeaderParseError {
    InvalidMagic,
    InvalidBitness(u8),
    InvalidEndianess,
    InvalidVersion(u8),
    InvalidAbi(u8),
    InvalidObjType(u16),
    InvalidASI,
    ReservedASI,
    InvalidVersionOrig(u32),
    InvalidSize(u16),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramHeaderParseError {
    InvalidProgramType(u32),
    InvalidFlags,
}

impl<T: BitFlag> From<FromBitsError<T>> for ProgramHeaderParseError {
    fn from(_value: FromBitsError<T>) -> Self {
        ProgramHeaderParseError::InvalidFlags
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SectionHeaderParseError {
    InvalidSectionType(u32),
    InvalidFlags,
    InvalidSectionName(FromUtf8Error),
}

impl<T: BitFlag> From<FromBitsError<T>> for SectionHeaderParseError {
    fn from(_value: FromBitsError<T>) -> Self {
        SectionHeaderParseError::InvalidFlags
    }
}

#[derive(Debug)]
pub enum ElfParseError {
    ElfHeaderParseError(ElfHeaderParseError),
    ProgramHeaderParseHeader(ProgramHeaderParseError),
    SectionHeaderParseError(SectionHeaderParseError),
    InvalidNameSecionType,
    SectionNotFound,
}

impl From<ElfHeaderParseError> for ElfParseError {
    fn from(value: ElfHeaderParseError) -> Self {
        Self::ElfHeaderParseError(value)
    }
}

impl From<ProgramHeaderParseError> for ElfParseError {
    fn from(value: ProgramHeaderParseError) -> Self {
        Self::ProgramHeaderParseHeader(value)
    }
}

impl From<SectionHeaderParseError> for ElfParseError {
    fn from(value: SectionHeaderParseError) -> Self {
        Self::SectionHeaderParseError(value)
    }
}

impl From<FromUtf8Error> for ElfParseError {
    fn from(value: FromUtf8Error) -> Self {
        Self::SectionHeaderParseError(SectionHeaderParseError::InvalidSectionName(value))
    }
}
